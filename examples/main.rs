extern crate redblack;

use redblack::Tree;

fn main() {
    {
        let mut tree: Tree<i32> = Tree::new();
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(1..100000);

        println!("Inserting 40 elements...");

        for i in 0..40 {
            tree.insert(i);
        }

        tree.print(|x| x.to_string());
    }

    {
        let mut tree: Tree<i32> = Tree::new();
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(1..100000);

        println!("Inserting 40 elements and deleting 20...");

        for i in 0..40 {
            tree.insert(i);
        }

        println!("Predecessor to 1: {:?}", tree.predecessor(1));
        println!("Predecessor to 29: {:?}", tree.predecessor(29));
        println!("Predecessor to 28: {:?}", tree.predecessor(28));
        println!("Successor to 28: {:?}", tree.successor(28));
        println!("Successor to 27: {:?}", tree.successor(27));
        println!("Successor to 40: {:?}", tree.successor(40));

        for i in 0..20 {
            tree.delete(i);
            tree.print(|x| format!("{:?}", x));
        }

        tree.print(|x| x.to_string());
    }

    {
        let mut tree: Tree<i32> = Tree::new();
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(1..100000);

        println!("Inserting 40 elements and deleting 40...");

        for i in 0..40 {
            tree.insert(i);
        }
        for i in 0..40 {
            tree.delete(i);
        }

        tree.print(|x| format!("{:?}", x));
    }
}
