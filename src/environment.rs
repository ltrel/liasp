use std::{collections::HashMap, cell::RefCell, rc::Rc };

use crate::{expression::Exp, math};

pub struct Environment {
    root: EnvLink
}

type EnvLink = Option<Rc<RefCell<EnvNode>>>;

struct EnvNode {
    bindings: HashMap<String, Exp>,
    parent: EnvLink,
}

impl Environment {
    pub fn new() -> Self {
        let env_node = EnvNode { bindings: HashMap::new(), parent: None };
        Environment { root: Some(Rc::new(RefCell::new(env_node))) }
    }

    pub fn parent(&self) -> Option<Self> {
        self.root.as_ref().map(|root| Environment { root: root.borrow().parent.clone() })
    }

    pub fn extend(&self) -> Self {
        let env_node = EnvNode {bindings: HashMap::new(), parent: self.root.clone() };
        Environment { root: Some(Rc::new(RefCell::new(env_node))) }
    }

    pub fn define(&mut self, ident: &str, val: &Exp) -> Result<(), String> {
        // Attempting to define on an environment with no root node is nonsense
        let mut borrow = self.root.as_ref().unwrap().borrow_mut();
        if borrow.bindings.contains_key(ident) {
            Err("Identifier already defined".to_owned())
        } else {
            borrow.bindings.insert(ident.to_owned(), val.clone());
            Ok(())
        }
    }

    pub fn assign(&mut self, ident: &str, val: &Exp) -> Result<(), String> {
        let root_link = self.root.as_ref().ok_or("Identifier does not exist in environment".to_owned())?;
        let mut borrow = root_link.borrow_mut();
        if borrow.bindings.contains_key(ident) {
            borrow.bindings.insert(ident.to_owned(), val.clone());
            Ok(())
        } else {
            let mut parent = self.parent().ok_or("Identifier does not exist in environment".to_owned())?;
            parent.assign(ident, val)
        }
    }

    pub fn lookup(&self, ident: &str) -> Option<Exp> {
        self.root.as_ref().and_then(|root_link| {
            root_link.borrow().bindings.get(ident).cloned().or_else(|| {
                self.parent().and_then(|parent| parent.lookup(ident))
            })
        })
    }
}

pub fn build_global_env() -> Environment {
    let mut env = Environment::new();
    env.define("+", &Exp::Function(math::add)).unwrap();
    env.define("-", &Exp::Function(math::subtract)).unwrap();
    env.define("*", &Exp::Function(math::multiply)).unwrap();
    env.define("five", &Exp::Number(5.0)).unwrap();
    env
}

#[cfg(test)]
mod test {
    use super::Environment;
    use crate::expression::Exp;

    #[test]
    fn single_layer() {
        let mut env = Environment::new();

        env.define("x", &Exp::Number(8.0)).unwrap();
        let x = env.lookup("x").unwrap();
        assert!(matches!(x, Exp::Number(val) if val == 8.0));

        assert!(env.define("x", &Exp::Number(2.0)).is_err());

        assert!(env.lookup("y").is_none());
    }

    #[test]
    fn multi_layer() {
        let mut parent = Environment::new();
        parent.define("x", &Exp::Number(5.0)).unwrap();
        parent.define("y", &Exp::Number(3.0)).unwrap();

        let mut child = parent.extend();
        child.define("y", &Exp::Number(2.0)).unwrap();

        let x = child.lookup("x").unwrap();
        assert!(matches!(x, Exp::Number(val) if val == 5.0));
        let y = child.lookup("y").unwrap();
        assert!(matches!(y, Exp::Number(val) if val == 2.0));

        assert!(child.lookup("z").is_none());
    }
}
