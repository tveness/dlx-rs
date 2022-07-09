use std::collections::HashMap;
use std::fmt;
type Index = usize;

enum LinkType {
    Spacer,
    Item,
    OptionElement,
}

struct OptionElement {
    ulink: Index,
    dlink: Index,
    top: Index,
}

struct Spacer {
    ulink: Index,
    dlink: Index,
}

struct Item {
    ulink: Index,
    dlink: Index,
    rlink: Index,
    llink: Index,
    l: usize,
}

/// Implements the linked lists, which are structured in the following way
/// ```text
/// i0  ⟷  i1  ⟷  i2  ⟷  i3  ⟷  i4
///        ⥯      ⥯     ⥯     ⥯   s0
/// o1     ⦿      ⦿     ⥯     ⥯   s1
/// o2     ⥯      ⥯     ⦿     ⥯   s2
/// o3     ⥯      ⦿     ⥯     ⦿   s3
/// o4     ⦿      ⥯     ⥯     ⥯   s4
///        ⥯      ⥯     ⥯     ⥯
/// ```
/// where arrows denote links.
///
/// The spacers s0,..., also form a doubly circularly linked list.
/// 
/// i0 is the root node for the linked list of items i1,...,.i4
/// 
/// s0 is the root node for the spacers  which link vertically to each other
/// 
/// ⦿ denote the option elements which contain links up and down and also reference their "parent" item
/// 
/// We may set up and solve this problem with the following code
/// ```
///# use dlx_rs::solver::Solver;
/// // Create Solver with 4 items
/// let mut s = Solver::new(4);
/// // Add options
/// s.add_option("o1", &[1,2]);
/// s.add_option("o2", &[3]);
/// s.add_option("o3", &[2,4]);
/// s.add_option("o4", &[1]);
/// 
/// // Iterate through all solutions
/// for solution in s {
///     assert_eq!(solution, ["o2","o3","o4"]);
/// }
/// 
/// ```
pub struct Solver {
    elements: Vec<Box<dyn Link>>,
    items: Index,
    options: HashMap<Index, Vec<Index>>,
    l: usize,
    sol_vec: Vec<Index>,
    yielding: bool,
    idx: Index,
    names: Vec<String>,
    spacer_ids: HashMap<Index, usize>,
    stage: Stage,
}
enum Stage {
    X2,
    X3,
    X5,
    X6,
    X8,
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // First write columns
        let mut last_col = 1;
        let mut linked_items = HashMap::new();
        let mut col_num = 0;

        write!(f, " ").unwrap();
        // First print only the linked items, and find their column numbers
        let mut index = self.elements[0].r();
        while index != 0 {
            linked_items.insert(index, col_num);
            col_num += 1;
            write!(f, "{} ", index).unwrap();
            index = self.elements[index].r();
        }

        //        println!("Linked items: {:?}",linked_items);
        //        println!("Linked items: {:?}",linked_items.keys());

        for i in self.elements.iter().skip(1 + self.items) {
            match i.link_type() {
                LinkType::Item => {}
                LinkType::Spacer => {
                    writeln!(f).unwrap();
                    last_col = 0;
                }
                LinkType::OptionElement => {
                    if let Some(&cur_col) = linked_items.get(&i.top()) {
                        //    println!("Cur_col: {}, last col: {}", cur_col, last_col);
                        let del = 2 * (1 + cur_col - last_col);
                        //    println!("del: {}",del);
                        write!(f, "{:del$}", i.top()).unwrap();
                        last_col = cur_col + 1;
                    };
                }
            };
        }

        write!(f, "")
    }
}
trait Link {
    fn u(&self) -> Index;
    fn d(&self) -> Index;
    fn r(&self) -> Index;
    fn l(&self) -> Index;
    fn set_u(&mut self, u: Index);
    fn set_d(&mut self, d: Index);
    fn set_r(&mut self, u: Index);
    fn set_l(&mut self, d: Index);
    fn link_type(&self) -> LinkType;
    fn top(&self) -> Index;
    fn inc_l(&mut self);
    fn dec_l(&mut self);
    fn get_l(&self) -> usize;
}

impl Link for Spacer {
    fn r(&self) -> Index {
        0
    }
    fn l(&self) -> Index {
        0
    }
    fn u(&self) -> Index {
        self.ulink
    }
    fn d(&self) -> Index {
        self.dlink
    }
    fn set_r(&mut self, _u: Index) {}
    fn set_l(&mut self, _u: Index) {}
    fn set_u(&mut self, u: Index) {
        self.ulink = u;
    }
    fn set_d(&mut self, d: Index) {
        self.dlink = d;
    }
    fn link_type(&self) -> LinkType {
        LinkType::Spacer
    }
    fn top(&self) -> Index {
        0
    }
    fn inc_l(&mut self) {}
    fn dec_l(&mut self) {}
    fn get_l(&self) -> usize {
        0
    }
}
impl Link for OptionElement {
    fn set_r(&mut self, _u: Index) {}
    fn set_l(&mut self, _u: Index) {}
    fn r(&self) -> Index {
        0
    }
    fn l(&self) -> Index {
        0
    }
    fn d(&self) -> Index {
        self.dlink
    }
    fn u(&self) -> Index {
        self.ulink
    }
    fn set_u(&mut self, u: Index) {
        self.ulink = u;
    }
    fn set_d(&mut self, d: Index) {
        self.dlink = d;
    }
    fn link_type(&self) -> LinkType {
        LinkType::OptionElement
    }
    fn top(&self) -> Index {
        self.top
    }
    fn inc_l(&mut self) {}
    fn dec_l(&mut self) {}
    fn get_l(&self) -> usize {
        0
    }
}
impl Link for Item {
    fn r(&self) -> Index {
        self.rlink
    }
    fn l(&self) -> Index {
        self.llink
    }
    fn d(&self) -> Index {
        self.dlink
    }
    fn u(&self) -> Index {
        self.ulink
    }
    fn set_u(&mut self, u: Index) {
        self.ulink = u;
    }
    fn set_d(&mut self, d: Index) {
        self.dlink = d;
    }
    fn set_r(&mut self, u: Index) {
        self.rlink = u
    }
    fn set_l(&mut self, u: Index) {
        self.llink = u
    }
    fn link_type(&self) -> LinkType {
        LinkType::Item
    }
    fn top(&self) -> Index {
        0
    }

    fn inc_l(&mut self) {
        self.l += 1;
    }
    fn dec_l(&mut self) {
        self.l -= 1;
    }
    fn get_l(&self) -> usize {
        self.l
    }
}

impl Solver {
    pub fn new(n: Index) -> Self {
        // First add null at element 0 (allows us to traverse items list)
        let mut elements: Vec<Box<dyn Link>> = vec![Box::new(Item {
            ulink: 0,
            dlink: 0,
            rlink: 1,
            llink: n,
            l: 0,
        })];
        // Now add items
        for i in 1..=n {
            let rlink = match i {
                _ if i < n => i + 1,
                _ if i == n => 0,
                _ => panic!("Invalid index"),
            };
            elements.push(Box::new(Item {
                ulink: i,
                dlink: i,
                llink: i - 1,
                rlink,
                l: 0,
            }));
        }

        // Add first spacer
        let spacer_index = elements.len();
        assert_eq!(spacer_index, n + 1);
        let spacer = Spacer {
            ulink: spacer_index,
            dlink: spacer_index,
        };
        elements.push(Box::new(spacer));

        Solver {
            elements,
            items: n,
            options: HashMap::new(),
            l: 0,
            sol_vec: vec![],
            names: vec![],
            spacer_ids: HashMap::new(),
            yielding: true,
            idx: 0,
            stage: Stage::X2,
        }
    }

    /// Adds an option which would cover items defined by `option`, and with name `name
    /// Specifically if our problems looks like
    /// 
    /// ```text
    /// i0  ⟷  i1  ⟷  i2  ⟷  i3  ⟷  i4
    ///        ⥯      ⥯     ⥯     ⥯   s0
    /// o1     ⦿      ⦿     ⥯     ⥯   s1
    /// o2     ⥯      ⥯     ⦿     ⥯   s2
    /// o3     ⥯      ⦿     ⥯     ⦿   s3
    /// o4     ⦿      ⥯     ⥯     ⥯   s4
    ///        ⥯      ⥯     ⥯     ⥯
    /// ```
    /// then `add_option("o5", &[1,2])` would take it to
    /// ```text
    /// i0  ⟷  i1  ⟷  i2  ⟷  i3  ⟷  i4
    ///        ⥯      ⥯     ⥯     ⥯   s0
    /// o1     ⦿      ⦿     ⥯     ⥯   s1
    /// o2     ⥯      ⥯     ⦿     ⥯   s2
    /// o3     ⥯      ⦿     ⥯     ⦿   s3
    /// o4     ⦿      ⥯     ⥯     ⥯   s4
    /// o5     ⦿      ⦿     ⥯     ⥯   s5
    ///        ⥯      ⥯     ⥯     ⥯
    /// ```
    pub fn add_option(&mut self, name: &str, option: &[Index]) {
        // Increase max depth, come back to this later
        self.sol_vec.push(0);
        //        self.sol_vec.push(0);

        // Now add elements from the option

        for &item_id in option {
            let new_ulink = self.elements[item_id].u();
            let new_id = self.elements.len();
            self.elements[new_ulink].set_d(new_id);
            self.elements[item_id].set_u(new_id);
            self.elements[item_id].inc_l();
            let new_node = OptionElement {
                ulink: new_ulink,
                dlink: item_id,
                top: item_id,
            };

            self.elements.push(Box::new(new_node));
        }

        //Add spacer at the end
        //Create new spacer
        let spacer_index = self.elements.len();
        let root_spacer_index = self.items + 1;
        let bottom_spacer_index = self.elements[root_spacer_index].u();
        let new_spacer = Spacer {
            dlink: root_spacer_index,
            ulink: bottom_spacer_index,
        };
        self.elements.push(Box::new(new_spacer));
        // Patch old spacers
        //Old bottom dlink = new spacer
        self.elements[bottom_spacer_index].set_d(spacer_index);
        // Patch root ulink
        self.elements[root_spacer_index].set_u(spacer_index);

        // Add the entry to the hash table
        self.options.insert(spacer_index, option.to_vec());
        self.names.push(String::from(name));
        self.spacer_ids.insert(spacer_index, self.names.len() - 1);
    }

    /// Covers item in column `i`
    /// i.e. `cover(2)` would transform
    /// 
    /// ```text
    /// i0  ⟷  i1  ⟷  i2  ⟷  i3  ⟷  i4
    ///        ⥯      ⥯     ⥯     ⥯   s0
    /// o1     ⦿      ⦿     ⥯     ⥯   s1
    /// o2     ⥯      ⥯     ⦿     ⥯   s2
    /// o3     ⥯      ⦿     ⥯     ⦿   s3
    /// o4     ⦿      ⥯     ⥯     ⥯   s4
    ///        ⥯      ⥯     ⥯     ⥯
    /// ```
    /// into
    /// 
    /// ```text
    /// i0  ⟷  i1  ⟷  ⟷  ⟷  i3  ⟷  i4
    ///        ⥯            ⥯     ⥯   s0
    /// o1     ⦿            ⥯     ⥯   s1
    /// o2     ⥯            ⦿     ⥯   s2
    /// o3     ⥯            ⥯     ⦿   s3
    /// o4     ⦿            ⥯     ⥯   s4
    ///        ⥯            ⥯     ⥯
    /// ```
    pub fn cover(&mut self, i: Index) -> Result<(), &'static str> {
        let col = &mut self.elements[i];
        match col.link_type() {
            LinkType::Item => {}
            _ => return Err("Can only cover items"),
        };
        // Hide all of the options in col i
        let mut p = col.d();
        while p != i {
            self.hide(p)?;
            p = self.elements[p].d();
        }

        // Unlink item
        self.unlink_item(i);
        //let l = self.elements[i].l();
        //let r = self.elements[i].r();
        //self.elements[l].set_r(r);
        //self.elements[r].set_l(l);

        Ok(())
    }

    fn unlink_item(&mut self, i: Index) {
        let l = self.elements[i].l();
        let r = self.elements[i].r();
        self.elements[l].set_r(r);
        self.elements[r].set_l(l);
    }

    fn relink_item(&mut self, i: Index) {
        let l = self.elements[i].l();
        let r = self.elements[i].r();
        self.elements[l].set_r(i);
        self.elements[r].set_l(i);
    }

    fn hide(&mut self, p: Index) -> Result<(), &'static str> {
        let mut q = p + 1;
        while q != p {
            let x = self.elements[q].top();
            let u = self.elements[q].u();
            let d = self.elements[q].d();

            match self.elements[q].link_type() {
                LinkType::Item => return Err("Hide encountered and item"),
                LinkType::Spacer => q = u,
                LinkType::OptionElement => {
                    self.elements[u].set_d(d);
                    self.elements[d].set_u(u);
                    self.elements[x].dec_l();
                }
            };
            q += 1;
        }

        Ok(())
    }

    /// Reverse of function [cover](crate::solver::Solver::cover)
    pub fn uncover(&mut self, i: Index) -> Result<(), &'static str> {
        // Relink item
        self.relink_item(i);
        //let l = self.elements[i].l();
        //let r = self.elements[i].r();
        //self.elements[l].set_r(i);
        //self.elements[r].set_l(i);

        let col = &mut self.elements[i];

        match col.link_type() {
            LinkType::Item => {}
            _ => return Err("Can only uncover items"),
        };

        // Hide all of the options in col i
        let mut p = col.u();
        while p != i {
            self.unhide(p)?;
            p = self.elements[p].u();
        }

        Ok(())
    }

    fn unhide(&mut self, p: Index) -> Result<(), &'static str> {
        let mut q = p - 1;
        while q != p {
            let x = self.elements[q].top();
            let u = self.elements[q].u();
            let d = self.elements[q].d();

            match self.elements[q].link_type() {
                LinkType::Item => return Err("Hide encountered and item"),
                LinkType::Spacer => q = d,
                LinkType::OptionElement => {
                    self.elements[u].set_d(q);
                    self.elements[d].set_u(q);
                    self.elements[x].inc_l();
                }
            };
            q -= 1;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn solve(&mut self) -> Option<Vec<String>> {
        // Follows stages of algorithm description in Fasc 5c, Knuth
        loop {
            match self.stage {
                Stage::X2 => match self.x2() {
                    Some(z) => return Some(z),
                    None => {}
                },
                Stage::X3 => {
                    self.x3x4();
                }
                Stage::X5 => {
                    self.x5();
                }
                Stage::X6 => {
                    self.x6();
                }
                Stage::X8 => match self.x8() {
                    true => {}
                    false => {
                        return None;
                    }
                },
            };
        }

        //        None

        //        self.x2()
    }

    pub fn output(&self) -> Vec<String> {

                let to_return = self
                    .sol_vec
                    .iter()
                    .take(self.l)
                    .map(|&x| self.spacer_for(x))
                    .map(|x| self.spacer_ids[&x])
                    .map(|x| self.names[x].clone())
                    .collect();
                    to_return

    }

    /// Stage X2 of Algorithm X
    /// If rlink(0) = 0, then all items are covered, so return current solution
    /// and also go to X8
    fn x2(&mut self) -> Option<Vec<String>> {
        //println!("State:");
        //println!("{}",self);
        //println!("RLINK: {}",self.elements[0].r());
        if self.elements[0].r() == 0 {
            if self.yielding {
                self.yielding = false;
                return Some(self.output());
            } else {
                self.yielding = true;
                self.stage = Stage::X8;
                return None;
            }
        }
        self.stage = Stage::X3;
        None
    }

    /// Stages X3 and X4 of algorithm X
    ///
    /// X3: Choose item `min_idx`, use MRV heuristic (i.e. smallest remaining value)
    ///
    /// X4: Cover item `min_idx`
    fn x3x4(&mut self) -> Option<Vec<String>> {
        // X3
        // Heuristic we choose is MRV

        // Walk along items and find minimum l
        let mut idx = self.elements[0].r();
        let mut min_idx = self.elements[0].r();
        let mut min_l = self.elements[idx].get_l();
        while idx != 0 {
            let l = self.elements[idx].get_l();
            if l < min_l {
                min_l = l;
                min_idx = idx;
            }
            idx = self.elements[idx].r();
        }

        // Now select the item which is covered by the minimum number of options
        self.idx = min_idx;

        // X4
        // Cover i

        //println!("Covering item X4: {}", self.idx);
        self.cover(self.idx).unwrap();

        // Set x_l <- DLINK(i)
        let x_l = self.elements[self.idx].d();

        // Save x_l in current guesses
        //     println!("self.l: {}",self.l);
        self.sol_vec[self.l] = x_l;

        self.stage = Stage::X5;
        None
        //        self.x5()
    }
    /// Stages X5 and X7 of Algorithm X
    ///
    /// Try x_l
    ///
    /// If x_l = i, then we are out of options and execute X7: backtrack
    ///
    /// Otherwise, cover all other items in option x_l, increase level and go back to X2
    ///
    fn x5(&mut self) -> Option<Vec<String>> {
        // X5
        // Try x_l
        // If x_l = i, then we are out of options and go to X7
        // Otherwise, cover all other items in option x_l, increase level and go back to X2
        //        println!("Partial sol: {:?}", &self.sol_vec[..self.l]);

        // Try xl
        let x_l = self.sol_vec[self.l];
        //        println!("Trying x_{}= {}", self.l, x_l);
        //        println!("idx: {}", self.idx);

        // If out of options (x_l reads downwards from self.idx, so have looped back around), backtrack
        if x_l == self.idx {
            // X7
            // Backtrack: Uncover item (i)

            //            println!("Uncovering X7: {}", x_l);
            self.uncover(x_l).unwrap();
            self.stage = Stage::X8;
            return None;
        }

        let mut p = x_l + 1;
        while p != x_l {
            //            println!("p: {}", p);
            let jt = self.elements[p].link_type();

            let j = self.elements[p].top();

            match jt {
                LinkType::Spacer => {
                    // If a spacer, then hop up one link
                    p = self.elements[p].u();
                }
                LinkType::OptionElement => {
                    //                    println!("Covering X5: {}", j);
                    //                    println!("State:");
                    //                    println!("{}", self);

                    self.cover(j).unwrap();
                }
                LinkType::Item => {
                    panic!("Trying an item {}", j);
                }
            };
            p += 1;
        }
        //        println!("--");

        self.l += 1;
        self.stage = Stage::X2;
        None
        //self.x2()
    }

    /// Stage X6 of Algorithm X
    ///
    /// Try again
    ///
    /// Uncover items != i in option x_l, then set x_l = DLINK(x_l): this is how we move through all of the options
    fn x6(&mut self) -> Option<Vec<String>> {
        let x_l = self.sol_vec[self.l];
        let mut p = x_l - 1;

        while p != x_l {
            let j = self.elements[p].top();
            if j == 0 {
                p = self.elements[p].d();
            } else {
                //                println!("Uncovering X6: {}",j);
                self.uncover(j).unwrap();
            }
            p -= 1;
        }
        self.idx = self.elements[x_l].top();
        self.sol_vec[self.l] = self.elements[x_l].d();

        self.stage = Stage::X5;
        None
        //        self.x5()
    }

    /// Stage X8 of Algorithm X
    /// Leave level l
    /// Terminate if l=0, otherwise l=l-1, go to X6
    fn x8(&mut self) -> bool {
        // X8
        match self.l {
            0 => false,
            _ => {
                self.l -= 1;
                self.stage = Stage::X6;
                true
                //                self.x6()
            }
        }
    }

    pub fn spacer_for(&self, x: Index) -> Index {
        let mut p = x;
        loop {
            match self.elements[p].link_type() {
                LinkType::Spacer => return p,
                LinkType::OptionElement => p += 1,
                LinkType::Item => panic!("Somehow ended up on an item"),
            };
        }
    }

    pub fn select(&mut self, name: &str) -> Result<(), &'static str> {
        // This selects an option by doing the followings

        // First get the spacer position of the option by firstly finding which
        // option it was
        let id = match self
            .names
            .clone()
            .iter()
            .position(|x| x == &name.to_string())
        {
            Some(z) => z,
            None => return Err("Invalid option specified"),
        };
        /*
        let mut id =0;
        for (i,item) in self.names.iter().enumerate() {
            if *item == name.to_string() {
                id = i;
                break;
            }
        }
        */
        // Now find the spacer id by going this many links down the chain
        // Start at root spacer node
        let mut spacer_id = self.items + 1;
        for _ in 0..id {
            spacer_id = self.elements[spacer_id].d();
        }
        //        println!("Spacer id: {}", spacer_id);

        // Now have the spacer node: cycle around and hide everything until we are at the next spacer mode
        let mut p = spacer_id + 1;

        loop {
            match self.elements[p].link_type() {
                LinkType::OptionElement => {
                    self.cover(self.elements[p].top()).unwrap();
                    p += 1;
                }
                LinkType::Spacer => break,
                LinkType::Item => break,
            };
        }

        Ok(())
    }
}

impl Iterator for Solver {
    type Item = Vec<String>;
    /// Produces next solution by following algorithm X
    /// as described in tAoCP in Fasc 5c, Dancing Links, Knuth
    ///
    /// Returns `Some` containing a vector of items if a solution remains, or
    /// `None` when no more solutions remaining
    fn next(&mut self) -> Option<Self::Item> {
        self.solve()
    }
}