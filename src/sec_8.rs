
// SMART POINTERS

pub fn smart_pointers() {
    // pointer variable is stored on the stack but the value 0.625 is on the heap.
    let single_value: Box<f64> = Box::new(0.625);
    let x: f64 = 0.625; // stored on the stack as primitive type.
    println!("Equal = {}", x == *single_value);

    let mut stack_var: i32 = 4;
    let stack_ref: &i32 = &stack_var;
    // println!("stack_var = {}, stack_ref = {}", stack_var, stack_ref);

    let heap_var: Box<i32> = Box::new(stack_var); 
    // here a copy of stack_var will be stored on the heap. heap_var will be stored on the stack and will point to the location on heap.\

    // Box pointer also owns the value it points to unlike the simple reference pointers.

    stack_var = 5;
    println!(
        "The value of stack_var = {} and heap_var = {}",
        stack_var, heap_var 
    ); // Here heap_var is still 4 because the value was initially copied and stored in heap.


    let point: Box<(i32, i32)> = Box::new((100, 20)); 
    // to access the attributes of the struct that the Box pointer points to, we dont need to deref.
    println!("{}, {}", point.0, point.1);

    // To access the value of the whole struct that Box pointer points to, we need to deref.
    let x: (i32, i32) = *point;

    /*
    let list: List = List::Cons(
        1, Box::new(List::Cons(
            2, Box::new(List::Cons(
                3, Box::new(List::Nil)
            ))
        ))
    );
    */

    let list: List = List::Cons(
        1, Some(Box::new(List::Cons(
            2, Some(Box::new(List::Cons(
                3, None
            )))
        )))
    );

    println!("{:?}", list);


    // Custom defined smart pointers.
    let a: i32 = 50;
    let b = Box::new(a);

    println!("{}", 50 == a);
    println!("{}", 50 == *b); // deref trait is implemented by Box here.
    // println!("{}", a == b);

    let sptr1: MySmartPointer = MySmartPointer::new(a);
    let sptr2: MySmartPointer = MySmartPointer::new(*b);

    println!("{}", a == *sptr1); // *(sptr1.deref())

    // Pointers are automaticlly dropped in reverse order unless explictly dropped. The rest will still drop in reverse.
    drop(sptr1);
    println!("{}", a == *sptr2);
    
}

/*

#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    Nil,
}

*/ // This is old example where we defined Nil

#[derive(Debug)]
enum List {
    Cons(i32,Option<Box<List>>)
}


// Custom smart pointer
struct MySmartPointer {
    value: i32
}

impl MySmartPointer {
    fn new(x: i32) -> MySmartPointer {
        MySmartPointer { value: x }
    }
}

/*
Standard library implementatio of Deref trait

pub trait Deref {
    type Target: name_of_type;
    fn deref(&self) -> &Self::Target;
}

*/

impl std::ops::Deref for MySmartPointer {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.value
    }
}


/*
Standard library implementatio of Drop trait

pub trait Drop {
    fn drop(&mut self);
}

*/
impl std::ops::Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("I am dying from memory! {:?}", self.value);
    }
}


pub fn linked_list() {

    let mut list: LinkedList<i32> = LinkedList::create_empty_list();

    list.add(5);
    list.add(3);
    list.add(1);
    list.add(2);
    list.remove();
    println!("{:?}", list.peek());
    list.print();
}

type ListPointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T>
where T: std::fmt::Debug + std::marker::Copy {
    element: T, 
    next: ListPointer<T>,
}


struct LinkedList<T>
where T: std::fmt::Debug + std::marker::Copy {
    head: ListPointer<T>,
}

impl<T> LinkedList<T>
where T: std::fmt::Debug + std::marker::Copy {

    fn create_empty_list() -> LinkedList<T> {
        LinkedList { head: None }
    }
    
    fn add(&mut self, element: T) {
        /* This implementation wont work becauuse match will transfer ownership so we will end up throwing self.head.
        match self.head {
            None => { 
                self.head = Some(Box::new(Node{element: element, next: None}));
            }

            Some(previous_head) => {
                let new_head: ListPointer = Some(Box::new(Node {
                    element: element, next: Some(previous_head)
                }));
                self.head = new_head;
            }
        };
        */
        
        let previous_head: ListPointer<T> = self.head.take();
        let new_head:Box<Node<T>> = Box::new(Node {
            element: element, next: previous_head
        });

        self.head = Some(new_head); // we are adding to the front of the list here.
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head: ListPointer<T> = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            },
            None => None
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(head) => Some(head.element),
            None => None
        }
    }

    fn print(&self) {
        let mut list_traversal: &ListPointer<T> = &self.head;

        loop {
            match list_traversal {
                Some(node) => {
                    println!("{:?}", node.element);
                    list_traversal = &node.next;
                },
                None => break,
            }
        }
    }

}


// GENERICS AND DEREF COERCIAN IN SMART POINTERS
pub fn deref_coersion() {   
    let sptr1: NewSmartPointer<&str> = NewSmartPointer::new("Suvineet Singh");
    my_fn(&sptr1); // Here rust will look at this variable and see if it implements Deref trait and try to deref itself
    // This is called deref coersion.

    // &newsmartpointer -> &str -> &str


    let some_vec: NewSmartPointer<Vec<i32>> = NewSmartPointer::new(vec![1, 2, 3]);
    for z in &*some_vec { // & is used to iterate references to each value without moving them to z.
        println!("The value is {}", z);
    }
}

struct NewSmartPointer<T> 
where T: std::fmt::Debug {
    value: T
}

impl<T> NewSmartPointer<T> 
where T: std::fmt::Debug {
    fn new(x: T) -> NewSmartPointer<T> {
        NewSmartPointer { value: x }
    }
}

impl<T> std::ops::Deref for NewSmartPointer<T>
where T: std::fmt::Debug {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> std::ops::Drop for NewSmartPointer<T>
where T: std::fmt::Debug {
    fn drop(&mut self) {
        println!("Dropping new smart pointer from memory. Value = {:?}", self.value);
    }
}

fn my_fn(str: &str) {
    println!("The string received is {}", str);
}


// Reference Counting Smart Pointer

use std::{rc::{Rc, Weak}, cell::RefCell};
enum AList {
    Cons(i32, Rc<AList>),
    Nil,
}

pub fn rc_pointers() {
    /*
    b --> 3 
           \
        a --> 1 -> 2 -> Nil
           /
    c --> 4
     */
    let a: Rc<AList> = Rc::new(AList::Cons(1, Rc::new(AList::Cons(2, Rc::new(AList::Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    {
        let b: Rc<AList> = Rc::new(AList::Cons(3, Rc::clone(&a))); // Here clone doesnt do a deep copy. Just incremenst the reference count
        println!("Count after creating b = {}", Rc::strong_count(&a));
        let c: Rc<AList> = Rc::new(AList::Cons(4, Rc::clone(&a)));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after code block = {}", Rc::strong_count(&a));

}


// REF CELLS
// Checks the borrowing rules at runtime not compile time.

pub fn doubly_linked_list() {
    let mut list1: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list1.remove_front();
    list1.push_front(32);
    list1.push_back(15);
    list1.push_front(5);
    list1.remove_back();
    list1.remove_front();
    list1.remove_front();
    list1.print();
}

struct DNode<T>
where T: std::fmt::Debug + std::marker::Copy {
    element: T,
    next: DListPointer<T>,
    prev: DListPointer<T>,
}

impl<T> DNode<T>
where T: std::fmt::Display + std::fmt::Debug + std::marker::Copy {
    fn new(element: T) -> Rc<RefCell<DNode<T>>> {
        Rc::new(RefCell::new(DNode {
            element: element,
            prev: None,
            next: None,
        }))
    }
}

impl<T> DoubleLinkedList<T>
where T: std::fmt::Display + std::fmt::Debug + std::marker::Copy {
    fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_head: Rc<RefCell<DNode<T>>> = DNode::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            },
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail: Rc<RefCell<DNode<T>>> = DNode::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            },
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove_front(&mut self) {
        if self.head.is_none() {
            println!("The list is empty");
        } else {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take(); //here prev is essentially set to None
                        self.head = Some(new_head);
                        self.head.clone()
                    },
                    None => {
                        self.tail.take();
                        println!("List is empty after removal!");
                        None
                    }
                }
            });
        }
    }

    fn remove_back(&mut self) {
        if self.tail.is_none() {
            println!("The list is empty");
        } else {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    } ,
                    None => {
                        self.head.take();
                        println!("List is empty after removal!");
                        None
                    }
                }
            });
        }
    }

    fn print(&self) {
        if self.head.is_none() {
            println!("[]");
        } else {
            let mut traversal: DListPointer<T> = self.head.clone();
            while !traversal.is_none() {
                print!("{} ", traversal.as_ref().unwrap().borrow().element);
                traversal = traversal.unwrap().borrow().next.clone();
            }
            println!();
        }
    }

}

type DListPointer<T> = Option<Rc<RefCell<DNode<T>>>>;

struct DoubleLinkedList<T>
where T: std::fmt::Debug + std::marker::Copy {
    head: DListPointer<T>,
    tail: DListPointer<T>,
}


// REFERENCE CYCLES CAUSING MEMORY LEAKS

// Smart pointers are only dropped when their strong_count is 1.
// So if we end up creating a cycle of refrences we will have a memory leak as they will not be dropped.

// We can use the rc::Weak rather than RefCell which has a upgrade and downgrade function.
// This allows to set weak references to a variable, which means the strong_count is not incremented,
// only the weak_count is. However, a weak reference means that ownership is not shared with the new variable
// that is referring to the original one.

// However, using weak references, even if we made a reference cycle, the variables will still be dropped
// because their strong_count will be 1.

// We can use upgrade on Weak to set a owned reference (incrementing the strong_count)


// Here we keep track of parents in a tree data structure.
// Each Node in a Tree can have Parent nodes and some child nodes.
#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>, // We want Parent to have ownership of Child
    // We want to track parent nodes so that a node is aware of it's parent 
    // and also a node's parent might change so we have the parent wrapped inside the ref cell smart pointer.
    children: RefCell<Vec<Rc<TreeNode>>>,
    // A node can have multiple children hence wrapped in a vector.
    // A node should be able to modify its children hence wrapping in RefCell.

    // Node should NOT take ownership of its parent, but rather should be aware of it, hence using a Weak Node.
}

pub fn weak_and_strong_refs() {

    // A leaf node. i.e. no further children.
    let leaf: Rc<TreeNode> = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()), // new empty Weak pointer
        children: RefCell::new(vec![]), // empty as no children
    });

    // 
    let branch: Rc<TreeNode> = Rc::new(TreeNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // The leaf is the only child here.
        // Since the parents are supposed to take ownership of their children, so we use the rc.clone() function
        // i.e. as long as the leaf (child) is alive, the parent is guaranteed to be alive since its strong_count
        // will be incremented by the branch which is the parent of the leaf at the end.
    });

    // Here we set the parent of leaf.
    // The branch is pointing towards the leaf (child) and the child is pointing towards branch (parent)
    // using a weak pointer. This would have been a cycle causing memory leak if we didnt use a weak pointer.
    
    // The child doesnt have ownership of its parent.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}


