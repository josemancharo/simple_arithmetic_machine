use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    ast::user_functions::UserFunctionDefinition,
    errors::{ErrorWithMessage, SamError},
    util::hash_str::hash_str,
};

use super::{
    builtin_functions::{setup_builtins, Func},
    constants::generate_constants,
    data_types::{Real, SamObject, SamValue},
    heap::SamHeap,
};

pub struct SamVM {
    pub(crate) stacks: Vec<Vec<SamValue>>,
    pub(crate) heap: SamHeap,
    pub(crate) current_stack: usize,
    pub(crate) current_scope: usize,
    pub(crate) constants: HashMap<u64, Real>,
    pub(crate) user_vars: Vec<HashMap<u64, SamValue>>,
    pub(crate) builtin_functions: HashMap<u64, Func>,
    pub(crate) user_functions: HashMap<u64, UserFunctionDefinition>,
}

impl SamVM {
    pub fn new() -> SamVM {
        return SamVM {
            stacks: vec![vec![]],
            current_stack: 0,
            current_scope: 0,
            constants: generate_constants(),
            user_functions: HashMap::new(),
            user_vars: vec![HashMap::new()],
            builtin_functions: setup_builtins(),
            heap: SamHeap::new(),
        };
    }

    pub fn register_function(&mut self, name: &str, func: Func) {
        self.builtin_functions.insert(hash_str(name.clone()), func);
    }

    pub(crate) fn heap_alloc(&mut self, obj: SamObject){
        let uuid = self.heap.alloc(obj);
        self.push_stack(SamValue::Reference(uuid));
    }

    pub(crate) fn set_var(&mut self, key: u64, value: SamValue) {
        self.user_vars[self.current_scope].insert(key, value);
    }

    pub(crate) fn pop_stack(&mut self) -> Result<SamValue, SamError> {
        return self.stacks[self.current_stack]
            .pop()
            .ok_or(ErrorWithMessage::new_box("stack empty!"));
    }

    pub(crate) fn pop_two(&mut self) -> Result<(SamValue, SamValue), SamError> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        return Ok((b, a));
    }

    pub(crate) fn pop_three(&mut self) -> Result<(SamValue, SamValue, SamValue), SamError> {
        let (c, b) = self.pop_two()?;
        let a = self.pop_stack()?;
        return Ok((c, b, a));
    }

    pub(crate) fn diadic_op(&mut self, op: fn(Real, Real) -> Real) -> Result<(), SamError> {
        let (b, a) = self.pop_two()?;
        if let (SamValue::Real(b), SamValue::Real(a)) = (b, a) {
            self.push_stack(SamValue::Real(op(a, b)));
            Ok({})
        }
        else {
            Err(ErrorWithMessage::new_box("diadic op must be applied between two real values"))
        }
    }

    pub(crate) fn monadic_op(&mut self, op: fn(Real) -> Real) -> Result<(), SamError> {
        let a = self.pop_stack()?;
        if let SamValue::Real(a) = a{
            self.push_stack(SamValue::Real(op(a)));
            Ok({})
        }
        else {
            Err(ErrorWithMessage::new_box("monadic op must be applied between two real values"))
        }
    }

    pub(crate) fn push_stack(&mut self, val: SamValue) {
        self.stacks[self.current_stack].push(val);
    }

    pub(crate) fn get_var(&mut self, key: u64) -> SamValue {
        let global = self.constants.get(&key);
        if let Some(val) = global {
            return SamValue::Real(val.clone());
        } else {
            let mut scope = self.current_scope;
            loop {
                let local = self.user_vars[scope].get(&key);
                if let Some(val) = local {
                    return *val;
                }
                if scope != 0 {
                    scope -= 1;
                }
                if scope == 0 {
                    break;
                }
            }
            return SamValue::default();
        }
    }
}
