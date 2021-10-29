use crate::ds;
use std::collections::VecDeque;

#[derive(Clone)]
struct _Layer<'a> {
    lines : Vec<ds::Line>,
    rects : Vec<(&'a Node, ds::Rect)>
}

pub struct Layer {
    pub lines : Vec<ds::Line>,
    pub texts : Vec<ds::Text>,
}

pub struct Node {
    pub children : Vec<Node>,
    pub value : f64,
    pub text : String
}

struct BestIterResult {
    value : usize,
    adj_area : f64,
    is_xy : bool
}

impl<'a> _Layer<'a> {
    fn new() -> _Layer<'a> {
        _Layer {
            lines : Vec::<ds::Line>::new(),
            rects : Vec::<(&Node, ds::Rect)>::new()
        }
    }
}

fn create_text(rect : &ds::Rect, msg : String) -> ds::Text {
    const MARGIN : f64 = 0.02; // 1% for all margins
    const CHAR_RATIO : f64 = 0.6; // a width/height ratio
    let ratio = CHAR_RATIO * msg.len() as f64;
    let text_rect = ds::Rect {
        a : rect.a,
        off : ds::Point {
            x : rect.off.x.min(rect.off.y * ratio),
            y : (rect.off.x.min(rect.off.y * ratio) / ratio) * 0.8
        }
    }.create_margin(MARGIN);
    
    ds::Text {
        a : text_rect.a,
        font_size : text_rect.off.y,
        text_length : text_rect.off.x,
        msg
    }
}

/// Composes a SVG layer from the given data using greedy recursive algorithm
pub fn compose(root_node : & Node, rect : &ds::Rect) -> Layer {
    let mut line_stroke_w = 4.0;
    let mut last_layer : _Layer = compose_greedy_rec(&root_node.children, rect,
        line_stroke_w);
    let mut layers = Vec::<_Layer>::new();
    let mut texts : Vec<ds::Text> = Vec::<ds::Text>::new();
    loop {
        line_stroke_w /= 3.0;
        let a = last_layer.rects.iter().filter(|x| !x.0.children.is_empty())
            .map(|x : &(& Node, ds::Rect)| 
                compose_greedy_rec(&x.0.children, &x.1, line_stroke_w))
            .collect::<Vec<_Layer>>();
        let iteration_layer = _Layer {    
            lines: a.iter().map(|x : &_Layer| x.lines.clone())
                    .flatten().collect::<Vec<_>>(),
            rects: a.iter().map(|x : &_Layer| x.rects.clone())
                    .flatten().collect::<Vec<_>>()
        };
        for (node, rect) in &last_layer.rects {
            if node.children.is_empty() {
                texts.push(create_text(rect, node.text.clone()));
            }
        }
        layers.push(iteration_layer.clone());
        last_layer = iteration_layer;
        if last_layer.rects.is_empty() {
            break;
        }
    }
    
    Layer {
        lines: layers.iter().map(|x : &_Layer| x.lines.clone())
                .flatten().collect::<Vec<_>>(),
        texts
    }
}

/// Composes a SVG layer from the given data using greedy recursive algorithm
fn compose_greedy_rec<'a>(ordered_nodes : &'a [Node], rect : &ds::Rect, 
    line_stroke_w : f64)
 -> _Layer<'a> {
    let total_count = ordered_nodes.len();
    if total_count < 1 {
        return _Layer::new();
    }
    let node = &ordered_nodes[0];
    if total_count == 1 { 
        let mut layer = _Layer::new();
        layer.rects.push((node, rect.clone()));
        return layer;
    }
    assert!(rect.off.x.is_normal());
    assert!(rect.off.y.is_normal());
    let rect_width = rect.off.x.min(rect.off.y);
    let node_length = node.value / rect_width;
    if total_count == 2 || node_length >= rect_width {
        // Change '>=' to '>' to regulate the polarity of the first line 
        // (in a square)
        let v = rect.off.x >= rect.off.y;
        let line = ds::Line {
          a: rect.a + (ds::Point { x: node_length, y: node_length }).mask(!v),
          off: (ds::Point { x: rect_width, y: rect_width }).mask(v),
          stroke_w: line_stroke_w
        };
        let local_rect = ds::Rect {
            a: rect.a,
            off: line.a + line.off - rect.a
        };
        let mut result = compose_greedy_rec(&ordered_nodes[1..], 
            &ds::Rect { a: line.a, off: rect.a + rect.off - line.a}, 
            line_stroke_w);
        result.lines.push(line);
        result.rects.push((node, local_rect));
        return result;
    }
    
    let best = adj_rem_iter(ordered_nodes, rect);
    let node_off = if best.is_xy { ds::Point {
                x: ((best.adj_area + node.value) / rect.off.y),
                y: node.value / ((best.adj_area + node.value) / rect.off.y) } } 
            else { ds::Point {
                y: ((best.adj_area + node.value) / rect.off.x),
                x: node.value / ((best.adj_area + node.value) / rect.off.x) }
            };
    let rem_line = ds::Line {
        a: rect.a + node_off.mask(!best.is_xy),
        off: rect.off.mask(best.is_xy),
        stroke_w: line_stroke_w
    };
    let adj_line = ds::Line {
        a: rect.a + node_off.mask(best.is_xy),
        off: node_off.mask(!best.is_xy),
        stroke_w: line_stroke_w
    };
    let rem = &ds::Rect { a: rem_line.a, off: rect.a + rect.off - rem_line.a };
    let adj = &ds::Rect { a: adj_line.a, off: if best.is_xy { 
        ds::Point { x: adj_line.off.x, y: rect.off.y - node_off.y } } else {
            ds::Point { y: adj_line.off.y, x: rect.off.x - node_off.x }
        }
    };
    let mut result_adj =
        if best.adj_area == 0.0 { 
            _Layer::new() 
        } else { 
            compose_greedy_rec(&ordered_nodes[1..(best.value+1)], adj,
                line_stroke_w) 
        };
    let local_rect = ds::Rect {
        a: rect.a,
        off: adj_line.a + adj_line.off - rect.a
    };
    result_adj.rects.push((node, local_rect));
    result_adj.lines.push(rem_line); // warning: prone to line overlapping
    if best.adj_area > 0.0 {
        result_adj.lines.push(adj_line);
    }
    let result_rem = 
        if total_count - best.value <= 2 {
            _Layer::new()
        } else {
            compose_greedy_rec(&ordered_nodes[(best.value+1)..], rem, 
                line_stroke_w)
        };
    result_adj.lines.extend(result_rem.lines.iter().cloned());
    result_adj.rects.extend(result_rem.rects.iter().cloned());
    result_adj
}

fn process_order_nodes_rec(nodes : &mut Vec<Node>, ratio : f64) {
    nodes.sort_by(|a : &Node, b : &Node| 
        if a.value > b.value { std::cmp::Ordering::Less }
        else { std::cmp::Ordering::Greater } ); // reverse sort by x.value
    for node in nodes {
        node.value *= ratio;
        if !node.children.is_empty() {
            process_order_nodes_rec(&mut node.children, ratio);
        }
    }
}

/// Sorts nodes in a value-decreasing order and fits the value to an area of 
/// the rectangle. 
/// Applies to all subnodes through an recursive BFS traversal
pub fn process_order_nodes(root_node : &mut Node, rect : &ds::Rect) {
    let expected_value : f64 = rect.off.x * rect.off.y;
    let actual_value : f64 = root_node.value;
    let ratio : f64 = expected_value / actual_value;
    let mut queue = VecDeque::<&mut Node>::new();
    queue.push_back(root_node);
    
    process_order_nodes_rec(&mut root_node.children, ratio);
}

/// Inner function of finding the best result
fn adj_rem_iter(ordered_nodes : &[Node], rect : &ds::Rect)
-> BestIterResult {
    // 0---x Rotation X-Y:
    // |1| | 1: node
    // |-+3| 2: adj (adjacent heap; bigger squares)
    // |2| | 3: rem (remaining heap; smaller squares)
    // y-+-+ whole: rect
    //       1-2 line: node_x
    //       1-3 line: node_y
    // 
    // reversing ordered_nodes causes bigger squares to remain in rem
    // and the smaller ones in rem, although the function is not ready yet.
    //
    // See: https://www.desmos.com/calculator/1ldheassrr
    
    let mut adj_area = -ordered_nodes[0].value;
    // node-rem case checking
    // node-rem solutions using a single line sometimes produce better results
    // consider [30; 1; 1; 1; 1; 1; 1]
    let mut best_cost : f64 = rect.off.x.min(rect.off.y);
    let mut best_is_xy = rect.off.x >= rect.off.y;
    let mut best_value = 0;
    let mut best_adj_area = adj_area;
    let node = &ordered_nodes[0];
    for (idx, item) in ordered_nodes.iter().enumerate() {
        adj_area += item.value;
        let node_x_variants = [
            (adj_area + node.value) / rect.off.y,
            (adj_area + node.value) / rect.off.x];
        let mut b = false;
        for node_x in &node_x_variants {
            let node_y = node.value / node_x;
            let cost = node_x + node_y;
            if cost < best_cost {
                best_cost = cost;
                best_value = idx;
                best_adj_area = adj_area;
                best_is_xy = !b;
            }
            b = true;
        }
    }
    BestIterResult {
        value: best_value,
        adj_area: best_adj_area.max(0.0),
        is_xy: best_is_xy
    }
}