#![allow(dead_code)]

use vnode::*;

pub enum Patch {
    Create(u64, &'static str),
    Append(u64, u64),
}

pub fn diff_children(old: &Children, new: &Children) -> Vec<Patch> {
    let mut patches = Vec::new();

    let mut old_children = old.iter();
    let mut new_children = new.iter();

    let mut old_child = old_children.next();
    let mut new_child = new_children.next();
    
    while old_child.is_some() || new_child.is_some() {
        match (old_child, new_child) {
            (None, Some((_, new_vnode))) => patches.push(Patch::Create(new_vnode.key, new_vnode.tag)),
            _ => ()
        }
    }
    
    patches
}