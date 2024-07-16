// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub struct DropBomb {
    pub to_be_dropped: bool,
    pub defused_invoked: bool
}

impl DropBomb {
    //const mut check = true;
    //to_be_dropped_1 = true;
    
    pub fn new() -> DropBomb {
        let cur_bomb = DropBomb{
            to_be_dropped: true,
            defused_invoked: false
        };
        cur_bomb
    }

    pub fn drop(&mut self) {
        if self.to_be_dropped && !self.defused_invoked {
            panic!("Boom")
        }
    }

    pub fn defuse(&mut self) {
        self.to_be_dropped = false;
        self.defused_invoked = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.drop();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        println!("{}", bomb.to_be_dropped);
        bomb.drop();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
