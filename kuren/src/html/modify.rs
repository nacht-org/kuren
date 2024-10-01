use kuren_tree::NodeId;

use crate::Node;

use super::Html;

impl Html {
    /// Detaches a node from the HTML tree and returns it.
    ///
    /// # Arguments
    ///
    /// * `id` - The `NodeId` of the node to be detached.
    ///
    /// # Returns
    ///
    /// An `Option<Node>` which contains the detached node if the node with the given `id` exists,
    /// otherwise `None`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut html = Html::new();
    /// let node_id = html.add_node(Node::new("div"));
    /// let detached_node = html.detach(node_id);
    /// assert!(detached_node.is_some());
    /// ```
    pub fn detach(&mut self, id: NodeId) -> Option<Node> {
        let mut node_mut = self.tree.get_mut(id)?;
        node_mut.detach();
        Some(node_mut.value().clone())
    }
}
