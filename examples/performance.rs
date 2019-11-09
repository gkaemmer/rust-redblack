extern crate redblack;

use std::time::Instant;
use redblack::Tree;

fn main() {
    {
        let mut tree: Tree<i32> = Tree::new();
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(1..100000);

        println!("Inserting 100,000 elements...");

        let start = Instant::now();
        for i in 1..100000 {
            tree.insert(i);
        }
        let duration = start.elapsed();

        // tree.in_order(|x| println!("{}", x));

        // tree.print(|x| x.to_string());

        println!("Depth: {}", tree.depth());
        println!("Took: {:?}", duration);
    }

    {
        let mut tree: Tree<i32> = Tree::with_capacity(100000);
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(1..100000);
        //
        println!("Inserting 100,000 elements with pre-allocated tree...");

        let start = Instant::now();
        for i in 1..100000 {
            tree.insert(i);
        }
        let duration = start.elapsed();

        // tree.in_order(|x| println!("{}", x));

        // tree.print(|x| x.to_string());

        println!("Depth: {}", tree.depth());
        println!("Took: {:?}", duration);
    }

    {
        let mut tree: Tree<i32> = Tree::with_capacity(100000);
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(1..100000);
        //
        println!("Inserting 100,000 and deleting 50,000 elements...");

        let start = Instant::now();
        for i in 1..100000 {
            tree.insert(i);
        }

        for i in 1..50000 {
            tree.delete(i);
        }
        let duration = start.elapsed();

        let mut expected = 50000;
        tree.in_order(|x| {
            if *x != expected {
                panic!("Tree is out of order")
            }
            expected += 1;
        });

        println!("Depth: {}", tree.depth());
        println!("Took: {:?}", duration);
    }
}
