use std::borrow::Cow;
use std::collections::HashSet;

use ratatui::layout::Offset;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear, List, ListItem, ListState, StatefulWidgetRef, WidgetRef};

pub struct TreeState {
    pub selected: Option<String>,
    pub opened: HashSet<String>,
    list_state: ListState,

    flat_nodes_cache: Vec<NodeData>,
    flat_nodes_cache_key: Option<Vec<Vec<String>>>,
}

impl Default for TreeState {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeState {
    pub fn new() -> Self {
        TreeState {
            selected: None,
            opened: HashSet::new(),
            list_state: ListState::default(),

            flat_nodes_cache: vec![],
            flat_nodes_cache_key: None,
        }
    }

    pub fn with_opened(mut self, opened: HashSet<String>) -> Self {
        self.opened = opened;
        self
    }

    pub fn with_selected(mut self, selected: Option<String>) -> Self {
        self.selected = selected;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.list_state = self.list_state.with_offset(offset);
        self
    }

    pub fn selected(&self) -> Option<String> {
        self.selected.clone()
    }

    pub fn selected_mut(&mut self) -> &mut Option<String> {
        &mut self.selected
    }

    pub fn offset(&self) -> usize {
        self.list_state.offset()
    }

    pub fn offset_mut(&mut self) -> &mut usize {
        self.list_state.offset_mut()
    }

    pub fn open(&mut self, id: &str) {
        self.opened.insert(id.to_string());
    }

    pub fn open_all(&mut self, ids: HashSet<String>) {
        self.opened.extend(ids);
    }

    pub fn close(&mut self, id: &str) {
        self.opened.remove(id);
    }

    pub fn close_all(&mut self) {
        self.opened.clear();
    }

    pub fn toggle(&mut self, id: &str) {
        if self.opened.contains(id) {
            self.close(id);
        } else {
            self.open(id);
        }
    }
}

#[derive(Clone)]
pub struct Node<'a> {
    pub id: String,
    pub spans: Vec<Span<'a>>,
    pub children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(id: &str, spans: Vec<Span<'a>>, children: Vec<Node<'a>>) -> Node<'a> {
        Node {
            id: id.to_string(),
            spans,
            children,
        }
    }

    pub fn flatten(&self, opened: &HashSet<String>, prefix: &[String]) -> Vec<Vec<String>> {
        let mut path = prefix.to_vec();
        path.push(self.id.clone());
        let mut paths = vec![path.clone()];

        if opened.contains(&self.id) {
            for child in &self.children {
                paths.extend(child.flatten(opened, &path));
            }
        }

        paths
    }
}

pub struct NodeData {
    pub path: Vec<String>,
    line: Vec<(String, Style)>,
}

pub struct Tree<'a> {
    block: Option<Block<'a>>,
    root: Node<'a>,
    style: Style,
    highlight_style: Style,
}

impl<'a> Tree<'a> {
    pub fn new(root: Node<'a>) -> Self {
        Self {
            root,
            highlight_style: Style::default(),
            block: None,
            style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Tree<'a> {
        self.block = Some(block);
        self
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Tree<'a> {
        self.style = style.into();
        self
    }

    pub fn highlight_style<S: Into<Style>>(mut self, style: S) -> Tree<'a> {
        self.highlight_style = style.into();
        self
    }

    pub fn make_lines(
        node: &Node<'a>,
        state: &TreeState,
        prefix_for_root: &[(String, Style)],
        prefix_for_children: &[(String, Style)],
        path: &[String],
    ) -> Vec<NodeData> {
        let mut root_item: Vec<(String, Style)> = prefix_for_root.to_vec();

        if node.children.is_empty() {
            root_item.push(("─".to_string(), Style::default()));
        } else if state.opened.contains(&node.id) {
            root_item.push(("○".to_string(), Style::default()));
        } else {
            root_item.push(("●".to_string(), Style::default()));
        }

        root_item.push((" ".to_string(), Style::default()));
        for span in &node.spans {
            root_item.push((span.content.to_string(), span.style));
        }

        let mut new_path = path.to_vec();
        new_path.push(node.id.clone());

        let mut items: Vec<NodeData> = vec![NodeData {
            path: new_path.clone(),
            line: root_item,
        }];

        if state.opened.contains(&node.id) {
            for (i, child) in node.children.iter().enumerate() {
                let mut child_prefix_for_root = prefix_for_children.to_vec();
                if i == node.children.len().wrapping_sub(1) {
                    child_prefix_for_root.push(("╰".to_string(), Style::default()));
                } else {
                    child_prefix_for_root.push(("├".to_string(), Style::default()));
                }
                let mut child_prefix_for_children = prefix_for_children.to_vec();
                if i != node.children.len().wrapping_sub(1) {
                    child_prefix_for_children.push(("│".to_string(), Style::default()));
                } else {
                    child_prefix_for_children.push((" ".to_string(), Style::default()));
                }
                for spans_list in Self::make_lines(
                    child,
                    state,
                    &child_prefix_for_root,
                    &child_prefix_for_children,
                    &new_path,
                ) {
                    items.push(spans_list);
                }
            }
        }

        items
    }
}

impl<'a> StatefulWidgetRef for Tree<'a> {
    type State = TreeState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let paths = self.root.flatten(&state.opened, &[]);
        if state.flat_nodes_cache_key.as_ref() != Some(&paths) {
            state.flat_nodes_cache = Self::make_lines(&self.root, state, &[], &[], &[]);
            state.flat_nodes_cache_key = Some(paths);
        }

        let lines = &state.flat_nodes_cache;
        state.list_state.select(
            lines
                .iter()
                .position(|d| state.selected == Some(d.path.last().unwrap().clone())),
        );

        let items = lines.iter().map(|d| {
            ListItem::new(Line::from(
                d.line
                    .iter()
                    .map(|(content, style)| Span::styled(content, *style))
                    .collect::<Vec<_>>(),
            ))
        });
        let mut list = List::new(items)
            .style(self.style)
            .highlight_style(self.highlight_style);
        if let Some(block) = &self.block {
            list = list.block(block.clone());
        }

        // FIXME(itome): For re-calculating list offset, we call render_ref before getting the actual offset.
        StatefulWidgetRef::render_ref(&list, area, buf, &mut state.list_state);
        WidgetRef::render_ref(&Clear, area, buf);

        let mut indent = lines[state.list_state.offset()]
            .path
            .len()
            .saturating_sub(1);
        if let Some(selected) = state.list_state.selected() {
            let selected_indent = lines[selected].path.len().saturating_sub(5);
            indent = indent.min(selected_indent);
        };

        let items = lines.iter().enumerate().map(|(i, d)| {
            if i < state.list_state.offset()
                || i >= state.list_state.offset() + area.height as usize
            {
                return Line::from(
                    d.line
                        .iter()
                        .map(|(content, style)| Span::styled(content, *style))
                        .collect::<Vec<_>>(),
                );
            }

            let mut indent_rest = indent;

            let mut spans = vec![];
            for (content, style) in d.line.clone() {
                let span = Span::styled(content, style);
                if indent_rest == 0 {
                    spans.push(span);
                    continue;
                }
                if indent_rest > span.width() {
                    indent_rest -= span.width();
                    continue;
                }

                let content = span
                    .content
                    .to_string()
                    .chars()
                    .skip(indent_rest)
                    .collect::<String>();
                spans.push(span.content(content));
                indent_rest = 0;
            }

            Line::from(spans)
        });

        let mut list = List::new(items)
            .style(self.style)
            .highlight_style(self.highlight_style);
        if let Some(block) = &self.block {
            list = list.block(block.clone());
        }

        StatefulWidgetRef::render_ref(&list, area, buf, &mut state.list_state);
    }
}

impl<'a> Widget for Tree<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TreeState::new();
        StatefulWidgetRef::render_ref(&self, area, buf, &mut state);
    }
}

impl<'a> WidgetRef for Tree<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = TreeState::new();
        StatefulWidgetRef::render_ref(self, area, buf, &mut state);
    }
}

impl<'a> StatefulWidget for Tree<'a> {
    type State = TreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidgetRef::render_ref(&self, area, buf, state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use ratatui::widgets::{BorderType, Borders};

    #[test]
    fn test_flatten_node() {
        let mut hash_set = HashSet::new();
        hash_set.insert("root".to_string());
        hash_set.insert("node2".to_string());
        let node = Node::new(
            "root",
            vec![],
            vec![
                Node::new("node1", vec![], vec![]),
                Node::new(
                    "node2",
                    vec![],
                    vec![
                        Node::new("node3", vec![], vec![]),
                        Node::new("node4", vec![], vec![]),
                    ],
                ),
            ],
        );

        assert_eq!(
            node.flatten(&hash_set, &[]),
            vec![
                vec!["root"],
                vec!["root", "node1"],
                vec!["root", "node2"],
                vec!["root", "node2", "node3"],
                vec!["root", "node2", "node4"],
            ]
        );
    }

    #[test]
    fn test_tree() {
        let mut hash_set = HashSet::new();
        hash_set.insert("root".to_string());
        hash_set.insert("node2".to_string());
        let mut state = TreeState::new().with_opened(hash_set);

        let node = Node::new(
            "root",
            vec![Span::raw("root")],
            vec![
                Node::new(
                    "node1",
                    vec![Span::raw("node1")],
                    vec![Node::new("hidden", vec![Span::raw("hidden")], vec![])],
                ),
                Node::new(
                    "node2",
                    vec![Span::raw("node2")],
                    vec![
                        Node::new("node3", vec![Span::raw("node3")], vec![]),
                        Node::new("node4", vec![Span::raw("node4")], vec![]),
                    ],
                ),
                Node::new("node5", vec![Span::raw("node5")], vec![]),
            ],
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 6));
        StatefulWidgetRef::render_ref(&Tree::new(node), buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![
                "○ root              ",
                "├● node1            ",
                "├○ node2            ",
                "│├─ node3           ",
                "│╰─ node4           ",
                "╰─ node5            ",
            ])
        );
    }

    #[test]
    fn test_tree_with_deep_nodes() {
        let mut hash_set = HashSet::new();
        hash_set.insert("root".to_string());
        hash_set.insert("node2".to_string());
        hash_set.insert("node3".to_string());
        hash_set.insert("node4".to_string());
        hash_set.insert("node5".to_string());
        hash_set.insert("node6".to_string());

        let node = Node::new(
            "root",
            vec![Span::raw("root")],
            vec![
                Node::new(
                    "node1",
                    vec![Span::raw("node1")],
                    vec![Node::new("hidden", vec![Span::raw("hidden")], vec![])],
                ),
                Node::new(
                    "node2",
                    vec![Span::raw("node2")],
                    vec![Node::new(
                        "node3",
                        vec![Span::raw("node3")],
                        vec![Node::new(
                            "node4",
                            vec![Span::raw("node4")],
                            vec![Node::new(
                                "node5",
                                vec![Span::raw("node5")],
                                vec![Node::new(
                                    "node6",
                                    vec![Span::raw("node6")],
                                    vec![Node::new("node7", vec![Span::raw("node7")], vec![])],
                                )],
                            )],
                        )],
                    )],
                ),
                Node::new("node8", vec![Span::raw("node8")], vec![]),
            ],
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 9));
        let mut state = TreeState::new().with_opened(hash_set);
        StatefulWidgetRef::render_ref(&Tree::new(node), buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![
                "○ root              ",
                "├● node1            ",
                "├○ node2            ",
                "│╰○ node3           ",
                "│ ╰○ node4          ",
                "│  ╰○ node5         ",
                "│   ╰○ node6        ",
                "│    ╰─ node7       ",
                "╰─ node8            ",
            ])
        );
    }

    #[test]
    fn test_tree_with_block() {
        let mut hash_set = HashSet::new();
        hash_set.insert("root".to_string());
        hash_set.insert("node2".to_string());
        let mut state = TreeState::new().with_opened(hash_set);

        let node = Node::new(
            "root",
            vec![Span::raw("root")],
            vec![
                Node::new(
                    "node1",
                    vec![Span::raw("node1")],
                    vec![Node::new("hidden", vec![Span::raw("hidden")], vec![])],
                ),
                Node::new(
                    "node2",
                    vec![Span::raw("node2")],
                    vec![
                        Node::new("node3", vec![Span::raw("node3")], vec![]),
                        Node::new("node4", vec![Span::raw("node4")], vec![]),
                    ],
                ),
                Node::new("node5", vec![Span::raw("node5")], vec![]),
            ],
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 8));
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        StatefulWidgetRef::render_ref(
            &Tree::new(node).block(block),
            buffer.area,
            &mut buffer,
            &mut state,
        );
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![
                "╭──────────────────╮",
                "│○ root            │",
                "│├● node1          │",
                "│├○ node2          │",
                "││├─ node3         │",
                "││╰─ node4         │",
                "│╰─ node5          │",
                "╰──────────────────╯"
            ])
        );
    }

    #[test]
    fn test_tree_with_shifted() {
        let mut hash_set = HashSet::new();
        hash_set.insert("root".to_string());
        hash_set.insert("node2".to_string());
        hash_set.insert("node3".to_string());
        hash_set.insert("node4".to_string());
        hash_set.insert("node5".to_string());
        hash_set.insert("node6".to_string());

        let node = Node::new(
            "root",
            vec![Span::raw("root")],
            vec![
                Node::new(
                    "node1",
                    vec![Span::raw("node1")],
                    vec![Node::new("hidden", vec![Span::raw("hidden")], vec![])],
                ),
                Node::new(
                    "node2",
                    vec![Span::raw("node2")],
                    vec![Node::new(
                        "node3",
                        vec![Span::raw("node3")],
                        vec![Node::new(
                            "node4",
                            vec![Span::raw("node4")],
                            vec![Node::new(
                                "node5",
                                vec![Span::raw("node5")],
                                vec![Node::new(
                                    "node6",
                                    vec![Span::raw("node6")],
                                    vec![Node::new("node7", vec![Span::raw("node7")], vec![])],
                                )],
                            )],
                        )],
                    )],
                ),
                Node::new("node8", vec![Span::raw("node8")], vec![]),
            ],
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 3));
        let mut state = TreeState::new()
            .with_opened(hash_set.clone())
            .with_selected(Some("node6".to_string()))
            .with_offset(6);
        StatefulWidgetRef::render_ref(
            &Tree::new(node.clone()),
            buffer.area,
            &mut buffer,
            &mut state,
        );
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![
                "   ╰○ node6         ",
                "    ╰─ node7        ",
                "─ node8             ",
            ])
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 3));
        let mut state = TreeState::new()
            .with_opened(hash_set)
            .with_selected(Some("node8".to_string()))
            .with_offset(6);
        StatefulWidgetRef::render_ref(&Tree::new(node), buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![
                "│   ╰○ node6        ",
                "│    ╰─ node7       ",
                "╰─ node8            ",
            ])
        );
    }
}
