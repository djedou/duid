use std::cmp::Ordering;
use crate::core::{
    v_node::VirtualNode
};
use super::Patch;

pub(crate) type Indexes<MSG> = [VirtualNode<MSG>; 2];


pub(crate) fn diff<MSG>(old_node: &VirtualNode<MSG>, new_node: &VirtualNode<MSG>) -> usize /*Vec<Patch<MSG>>*/ 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    let old_node_pairs = to_indexes_pair(&old_node);
    let new_node_pairs = to_indexes_pair(&new_node);
    
    let added_branches = new_node_pairs.iter().filter(|n| !includes_indexes(&old_node_pairs, &n)).map(|arg| arg.to_owned()).collect::<Vec<Indexes<MSG>>>();
    let removed_branches = old_node_pairs.iter().filter(|o| !includes_indexes(&new_node_pairs, &o)).map(|arg| arg.to_owned()).collect::<Vec<Indexes<MSG>>>();
    
    let result = added_branches.iter().filter(|add| {
        match removed_branches.iter().find(|remove| remove[0] == add[0] && remove[1] != add[1]) {
            Some(_) => true,
            None => false
        }
    })
    .map(|arg| arg.to_owned())
    .collect::<Vec<Indexes<MSG>>>();

    crate::console::info!("diff: {:#?}", result);
    
    /*
    vec![
        Patch::AddedBranches(new_node_pairs.iter().filter(|n| !includes_indexes(&old_node_pairs, &n)).map(|arg| arg.to_owned()).collect::<Vec<Indexes<MSG>>>()),
        Patch::RemovedBranches(old_node_pairs.iter().filter(|o| !includes_indexes(&new_node_pairs, &o)).map(|arg| arg.to_owned()).collect::<Vec<Indexes<MSG>>>()),
        //Patch::AddedNodes(flat_unique(&new_node_pairs).iter().filter(|n| !flat_unique(&old_node_pairs).contains(n)).map(|arg| arg.to_owned()).collect::<Vec<VirtualNode<MSG>>>()),
        //Patch::RemovedNodes(flat_unique(&old_node_pairs).iter().filter(|o| !flat_unique(&new_node_pairs).contains(o)).map(|arg| arg.to_owned()).collect::<Vec<VirtualNode<MSG>>>()),
    ];
*/
    2
}

fn to_indexes_pair<MSG>(node: &VirtualNode<MSG>) -> Vec<Indexes<MSG>> 
 where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
 {
    let result: Vec<_> = node.children.iter().map(|child| {
        let mut indexes = vec![[node.make_copy(), child.make_copy()]];
        indexes.extend_from_slice(&to_indexes_pair(child));
        indexes
    })
    .collect();

    let mut res = result.into_iter()
        .fold(vec![], |mut old_vec: Vec<Indexes<MSG>>, new_vec: Vec<Indexes<MSG>>| {
            old_vec.extend_from_slice(&new_vec);
            old_vec
        });
        
    res.sort_by(|a, b| {
        match a[0].key.cmp(&b[0].key).is_lt() {
            true => Ordering::Less,
            false => a[1].key.cmp(&b[1].key)
        }
    });

    res
}

fn includes_indexes<MSG>(indexes_slices: &[Indexes<MSG>], index: &Indexes<MSG>) -> bool 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    match indexes_slices.iter().find(|item| item[0] == index[0] && item[1] == index[1]) {
        Some(_) => true,
        None => false
    }
}
/*
fn flat_unique<MSG>(indexes_slices: &[Indexes<MSG>]) -> Vec<VirtualNode<MSG>> 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    let mut data = indexes_slices.into_iter().flatten().map(|flat| flat.to_owned()).collect::<Vec<VirtualNode<MSG>>>();
    data.dedup();
    data
}
*/