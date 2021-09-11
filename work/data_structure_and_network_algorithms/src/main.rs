use std::cmp;

const HEAP_WIDTH: usize = 3;
const HEAP_DEPTH: usize = 10;

#[derive(Clone, Copy, Debug)]
struct Item{
    key: i64,
    value: i64
}

impl Item{
    fn new(key: i64, value: i64) -> Item{
        Item{
            key: key,
            value: value
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct  DHeap{
    value: [Option<Item>; HEAP_WIDTH ^ HEAP_DEPTH],
    size: usize
}

// x はノード番号
// key value
impl DHeap{
    // indexの補正
    fn get(&self, x: usize)-> Item{
        self.value[x-1].unwrap()
    }

    fn set(&mut self, x: usize, item: Item){
        self.value[x-1] = Some(item);
        self.size = cmp::max(self.size, x);
    }

    fn slice(&self, start: usize, end: usize)-> &[Option<Item>]{
        &self.value[start-1 .. end-1]
    }
    fn mv(&mut self, from: usize, to: usize){
        self.value[to-1] = self.value[from-1];
    }

    fn findmin(&self)-> Item{
        self.get(1)
    }

    fn shiftup(&mut self, i: Item, x: usize) {
        let mut p: usize;
        let mut _x: usize = x;
        p = (x - 1) / HEAP_DEPTH;
        while p != 0 &&  self.get(x).key > i.key{
            self.set(x, self.get(p));
            _x = p;
            p = (p-1)/ HEAP_DEPTH
        }
        self.set(x, i);
    }

    fn shiftdown(&mut self, i: Item,  mut x: usize){
        println!(" index {:?}", x);
        let mut c: usize;
        c = self.minchild(x);
        while c != 0 && self.get(c).key < i.key {
            self.mv(c, x);
            x = c;
            c = self.minchild(c);
        }
        self.set(x, i);
        println!("end shiftdown {:?}", self);
    }

    fn minchild(&self, x: usize) -> usize{
        if HEAP_DEPTH * ( x -1 )+2 > self.size {
            println!("{:?}", 0);
            return 0;
        }else{
            if HEAP_DEPTH * (x-1)+2<= self.size{
                let start = HEAP_DEPTH * (x -1)+ 2;
                let end = cmp::min::<usize>(HEAP_DEPTH * x + 1, self.size);
                println!("size {:?}", self.size);
                println!("start {:?}", start);
                println!("end {:?}", end);
                let index = (start..end).min_by_key(|x| self.get(*x).value);
                println!("{:?}", index);
                return index.unwrap();
            }
            panic!("why");
        }
    }
    
    fn new(data: &[Item])-> DHeap{
        let mut h = DHeap{
            value: [None;HEAP_WIDTH ^ HEAP_DEPTH],
            size: 0
        };
        
        for (x, each) in data.iter().enumerate() {
            h.set(x+1, *each)
        }
        println!("{:?}", h);
        for (x, each) in data.iter().enumerate() {
            h.shiftdown(*each, x+1)
        }
        h
    }
}



fn main() {
    let h = DHeap::new(&[Item::new(4,4), Item::new(1,1), Item::new(2,2)]);
    println!("{:?}", h);
    println!("Hello, world!");
}
