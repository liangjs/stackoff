use dflow::FlowState;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::*;

struct Analyser<Node>
where
    Node: StackOp,
{
    _phatom: PhantomData<Node>,
}

impl<Node> Analyser<Node>
where
    Node: StackOp,
{
    fn new() -> Analyser<Node> {
        Analyser {
            _phatom: Default::default(),
        }
    }
}

pub enum Error {
    MultiValue(NodeIndex),
}

#[derive(PartialEq, Eq, Clone)]
struct State<Offset>
where
    Offset: crate::Offset,
{
    data: Option<Offset>,
}

impl<Offset> dflow::FlowState for State<Offset>
where
    Offset: crate::Offset,
{
    fn empty() -> Self {
        State {
            data: Some(Offset::zero()),
        }
    }

    fn merge(&mut self, other: &Self) {
        if self != other {
            self.data = None;
        }
    }
}

impl<Node> dflow::Analysis for Analyser<Node>
where
    Node: crate::StackOp,
{
    type State = State<Node::Offset>;
    type NodeWeight = Node;

    fn entry_influx(&self) -> Self::State {
        Self::State::empty()
    }

    fn flow_through<NodeRef>(&self, node: NodeRef, influx: &Self::State) -> Self::State
    where
        NodeRef: petgraph::visit::NodeRef<Weight = Self::NodeWeight>,
    {
        let data = match influx.data.clone() {
            None => None,
            Some(influx) => {
                let node = node.weight();
                let change = node.pointer_change();
                Some(influx + change)
            }
        };
        Self::State { data }
    }
}

pub fn convert<Graph, Node>(
    graph: Graph,
    entry: NodeIndex,
) -> Result<HashMap<NodeIndex, Node::Offset>, Error>
where
    Graph: CFG<Node>,
    Node: StackOp,
{
    let analyser = Analyser::new();
    let result = dflow::analyse(&analyser, graph, entry);
    let mut ans = HashMap::new();
    for (index, flow) in result.iter() {
        let offset = flow.influx.data.as_ref();
        if offset.is_none() {
            return Err(Error::MultiValue(*index));
        }
        ans.insert(*index, offset.unwrap().clone());
    }
    Ok(ans)
}
