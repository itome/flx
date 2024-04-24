use std::collections::HashSet;

use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListItem, ListState, StatefulWidgetRef, WidgetRef};

pub struct TreeState {
    pub selected: Option<String>,
    pub opened: HashSet<String>,
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

    pub fn selected(&self) -> Option<String> {
        self.selected.clone()
    }

    pub fn selected_mut(&mut self) -> &mut Option<String> {
        &mut self.selected
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
        prefix_for_root: &[Span<'a>],
        prefix_for_children: &[Span<'a>],
    ) -> Vec<(String, Line<'a>)> {
        let mut root_item: Vec<Span> = prefix_for_root.to_vec();

        if node.children.is_empty() {
            root_item.push(Span::raw("─"));
        } else if state.opened.contains(&node.id) {
            root_item.push(Span::raw("○"));
        } else {
            root_item.push(Span::raw("●"));
        }

        root_item.push(Span::raw(" "));
        for span in &node.spans {
            root_item.push(span.clone());
        }

        let mut items: Vec<(String, Line<'a>)> = vec![(node.id.clone(), Line::from(root_item))];

        if state.opened.contains(&node.id) {
            for (i, child) in node.children.iter().enumerate() {
                let mut child_prefix_for_root = prefix_for_children.to_vec();
                if i == node.children.len().wrapping_sub(1) {
                    child_prefix_for_root.push(Span::raw("╰"));
                } else {
                    child_prefix_for_root.push(Span::raw("├"));
                }
                let mut child_prefix_for_children = prefix_for_children.to_vec();
                if i != node.children.len().wrapping_sub(1) {
                    child_prefix_for_children.push(Span::raw("│"));
                } else {
                    child_prefix_for_children.push(Span::raw(" "));
                }
                for spans_list in Self::make_lines(
                    child,
                    state,
                    &child_prefix_for_root,
                    &child_prefix_for_children,
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
        let lines = Self::make_lines(&self.root, state, &[], &[]);
        let items = lines.iter().map(|line| ListItem::new(line.1.clone()));
        let mut list = List::new(items)
            .style(self.style)
            .highlight_style(self.highlight_style);
        if let Some(block) = &self.block {
            list = list.block(block.clone());
        }
        let mut list_state = ListState::default().with_selected(
            lines
                .iter()
                .position(|line| state.selected == Some(line.0.clone())),
        );
        StatefulWidgetRef::render_ref(&list, area, buf, &mut list_state);
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
    use ratatui::{
        assert_buffer_eq,
        widgets::{BorderType, Borders},
    };

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
        assert_buffer_eq!(
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
        assert_buffer_eq!(
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
        assert_buffer_eq!(
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
}
