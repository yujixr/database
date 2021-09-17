use super::primitive::Primitive;
use crate::{Node, RootNode};
use std::{cmp, error::Error, fmt, marker};

pub trait SecondaryIndex<K, V, const N: usize>
where
    K: fmt::Debug,
{
    fn unique_name(&self) -> String;
    fn find(&self, key: &Primitive) -> Option<&Vec<K>>;
    fn select(&self, value: &V) -> Primitive;
    fn validate(&self, value: &Primitive) -> bool;
    fn append_to(&mut self, key: &Primitive, primary_key: K) -> Result<(), Box<dyn Error>>;
    fn remove_from(&mut self, key: &Primitive, primary_key: K) -> Result<(), Box<dyn Error>>;
}

pub struct DefaultSecondaryIndex<K1, V, K2, FnSelector, FnValidator, const N: usize>
where
    K1: fmt::Debug,
    K2: fmt::Debug,
    FnSelector: 'static + Fn(&V) -> Primitive,
    FnValidator: 'static + Fn(&Primitive) -> Option<&K2>,
{
    name: String,
    index: RootNode<K2, Vec<K1>, N>,
    selector: FnSelector,
    validator: FnValidator,
    phantom: marker::PhantomData<V>,
}

impl<K1, V, K2, FnSelector, FnValidator, const N: usize>
    DefaultSecondaryIndex<K1, V, K2, FnSelector, FnValidator, N>
where
    K1: 'static + fmt::Debug + Clone,
    K2: 'static + fmt::Debug + Clone + cmp::Ord,
    FnSelector: 'static + Fn(&V) -> Primitive,
    FnValidator: 'static + Fn(&Primitive) -> Option<&K2>,
{
    pub fn new(name: String, selector: FnSelector, validator: FnValidator) -> Self {
        Self {
            name,
            index: RootNode::new(),
            selector,
            validator,
            phantom: marker::PhantomData::<V>,
        }
    }
}

impl<K1, V, K2, FnSelector, FnValidator, const N: usize> SecondaryIndex<K1, V, N>
    for DefaultSecondaryIndex<K1, V, K2, FnSelector, FnValidator, N>
where
    K1: 'static + fmt::Debug + Clone + cmp::PartialEq,
    K2: 'static + fmt::Debug + Clone + cmp::Ord,
    FnSelector: 'static + Fn(&V) -> Primitive,
    FnValidator: 'static + Fn(&Primitive) -> Option<&K2>,
{
    fn unique_name(&self) -> String {
        self.name.clone()
    }

    fn find(&self, key: &Primitive) -> Option<&Vec<K1>> {
        if let Some(key) = (self.validator)(key) {
            self.index.find(key)
        } else {
            None
        }
    }

    fn select(&self, value: &V) -> Primitive {
        (self.selector)(value)
    }

    fn validate(&self, value: &Primitive) -> bool {
        match (self.validator)(value) {
            Some(_) => true,
            None => false,
        }
    }

    fn append_to(&mut self, key: &Primitive, primary_key: K1) -> Result<(), Box<dyn Error>> {
        if let Some(key) = (self.validator)(key) {
            match self.index.find(key) {
                Some(primary_keys) => {
                    let pointer = primary_keys as *const Vec<K1>;
                    let address = pointer as usize;
                    let pointer = address as *mut Vec<K1>;
                    unsafe { (*pointer).push(primary_key) }
                }
                None => self.index.insert(key, vec![primary_key])?,
            }
            Ok(())
        } else {
            todo!()
        }
    }

    fn remove_from(&mut self, key: &Primitive, primary_key: K1) -> Result<(), Box<dyn Error>> {
        if let Some(key) = (self.validator)(key) {
            if let Some(primary_keys) = self.index.find(key) {
                if primary_keys.len() == 1 {
                    self.index.remove(key)?;
                } else {
                    let pointer = primary_keys as *const Vec<K1>;
                    let address = pointer as usize;
                    let pointer = address as *mut Vec<K1>;
                    unsafe { (*pointer).retain(|x| *x != primary_key) }
                }
            }
            Ok(())
        } else {
            todo!()
        }
    }
}
