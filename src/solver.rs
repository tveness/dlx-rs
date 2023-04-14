use std::collections::HashMap;
use std::fmt;
type Index = usize;

#[derive(Clone, Debug)]
enum Link {
    Spacer(Spacer),
    Item(Item),
    OptionElement(OptionElement),
}

#[derive(Clone, Debug)]
struct OptionElement {
    ulink: Index,
    dlink: Index,
    top: Index,
}

#[derive(Clone, Debug)]
struct Spacer {
    ulink: Index,
    dlink: Index,
}

#[derive(Clone, Debug)]
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
///# use std::error::Error;
///# use dlx_rs::solver::Solver;
///# fn main() -> Result<(), Box<dyn Error>> {
/// // Create Solver with 4 items
/// let mut s = Solver::new(4);
/// // Add options
/// s.add_option("o1", &[1, 2])
///     .add_option("o2", &[3])
///     .add_option("o3", &[2, 4])
///     .add_option("o4", &[1]);
///
/// // Iterate through all solutions
/// if let Some(solution) = s.next() {
///     assert_eq!(solution, ["o2","o3","o4"]);
///     Ok(())
/// }
/// else {
///     Err("No solution found".into())
///     }
///# }
/// ```
#[derive(Clone)]
pub struct Solver {
    elements: Vec<Link>,
    items: Index,
    options: HashMap<Index, Vec<Index>>,
    l: usize,
    sol_vec: Vec<Index>,
    yielding: bool,
    idx: Index,
    names: Vec<String>,
    spacer_ids: HashMap<Index, usize>,
    stage: Stage,
    optional: Index,
}

/// enum used to determine which stage of the algorithm we are in
///
/// This approach avoids recursive function calls which, in very large problems, can cause a stack overflow
#[derive(Clone)]
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
            match *i {
                Link::Item(_) => {}
                Link::Spacer(_) => {
                    writeln!(f).unwrap();
                    last_col = 0;
                }
                Link::OptionElement(_) => {
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
impl Link {
    fn u(&self) -> Index {
        match self {
            Link::Spacer(x) => x.ulink,
            Link::OptionElement(x) => x.ulink,
            Link::Item(x) => x.ulink,
        }
    }
    fn d(&self) -> Index {
        match self {
            Link::Spacer(x) => x.dlink,
            Link::OptionElement(x) => x.dlink,
            Link::Item(x) => x.dlink,
        }
    }
    fn r(&self) -> Index {
        match self {
            Link::Spacer(_) => 0,
            Link::OptionElement(_) => 0,
            Link::Item(x) => x.rlink,
        }
    }
    fn l(&self) -> Index {
        match self {
            Link::Spacer(_) => 0,
            Link::OptionElement(_) => 0,
            Link::Item(x) => x.llink,
        }
    }
    fn set_u(&mut self, u: Index) {
        match self {
            Link::Spacer(x) => x.ulink = u,
            Link::OptionElement(x) => x.ulink = u,
            Link::Item(x) => x.ulink = u,
        }
    }
    fn set_d(&mut self, d: Index) {
        match self {
            Link::Spacer(x) => x.dlink = d,
            Link::OptionElement(x) => x.dlink = d,
            Link::Item(x) => x.dlink = d,
        }
    }
    fn set_r(&mut self, u: Index) {
        match self {
            Link::Spacer(_) => {}
            Link::OptionElement(_) => {}
            Link::Item(x) => x.rlink = u,
        }
    }
    fn set_l(&mut self, d: Index) {
        match self {
            Link::Spacer(_) => {}
            Link::OptionElement(_) => {}
            Link::Item(x) => x.llink = d,
        }
    }
    fn top(&self) -> Index {
        match self {
            Link::Spacer(_) => 0,
            Link::OptionElement(x) => x.top,
            Link::Item(_) => 0,
        }
    }
    fn inc_l(&mut self) {
        match self {
            Link::Spacer(_) => {}
            Link::OptionElement(_) => {}
            Link::Item(x) => x.l += 1,
        }
    }
    fn dec_l(&mut self) {
        match self {
            Link::Spacer(_) => {}
            Link::OptionElement(_) => {}
            Link::Item(x) => x.l -= 1,
        }
    }
    fn get_l(&self) -> usize {
        match self {
            Link::Spacer(_) => 0,
            Link::OptionElement(_) => 0,
            Link::Item(x) => x.l,
        }
    }
}
/*
impl Link for Spacer {
    fn clone_dyn(&self) -> Box<dyn Link> {
        Box::new(self.clone())
    }
}
*/

impl Solver {
    /// Returns a solver with `n` items, all of which must be covered exactly
    /// once
    pub fn new(n: Index) -> Self {
        Self::new_optional(n, 0)
    }

    /// Returns a solver with `n` mandatory items and `m` optional items to be covered
    /// This allows us to include items which may or may not be covered (but
    /// still may not be covered more than once)
    ///
    /// Example, where optional elements are after |
    /// ```text
    ///     i1  i2  i3  i4 | i5
    /// o1   1   0   1  0  |  0
    /// o2   0   1   0  1  |  0
    /// o3   1   0   0  0  |  1
    /// o4   0   0   1  0  |  0
    /// o5   0   0   1  0  |  1
    /// ```
    /// Here we can see taking \[o1,o2\] works, as does \[o2,o3,o4\], but *not*
    /// \[o2,o3,o5\], because then i4 would be double covered
    ///
    /// The code that does this is
    /// ```
    ///# use dlx_rs::solver::Solver;
    ///
    /// let mut s = Solver::new_optional(4,1);
    ///
    /// s.add_option("o1", &[1, 3])
    ///     .add_option("o2", &[2, 4])
    ///     .add_option("o3", &[1, 5])
    ///     .add_option("o4", &[3])
    ///     .add_option("o5", &[3, 5]);
    ///
    /// let s1 = s.next().unwrap_or_default();
    /// let s2 = s.next().unwrap_or_default();
    /// let s3 = s.next();
    /// assert_eq!(s1,["o2","o1"]);
    /// assert_eq!(s2,["o2","o3","o4"]);
    /// assert_eq!(s3,None);
    /// ```
    ///
    pub fn new_optional(mandatory: Index, opt: Index) -> Self {
        // optional stores the index where the optional parameters begin: this
        // is required for both checking completeness of solution (in step X2)
        // and also in choosing MRV (step X3)
        let optional = mandatory + 1;
        let n = mandatory + opt;
        // First add null at element 0 (allows us to traverse items list)
        let mut elements: Vec<Link> = vec![Link::Item(Item {
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
            elements.push(Link::Item(Item {
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
        let spacer = Link::Spacer(Spacer {
            ulink: spacer_index,
            dlink: spacer_index,
        });
        elements.push(spacer);

        Solver {
            optional,
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
    pub fn add_option(&mut self, name: &str, option: &[Index]) -> &mut Self {
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
            let new_node = Link::OptionElement(OptionElement {
                ulink: new_ulink,
                dlink: item_id,
                top: item_id,
            });

            self.elements.push(new_node);
        }

        //Add spacer at the end
        //Create new spacer
        let spacer_index = self.elements.len();
        let root_spacer_index = self.items + 1;
        let bottom_spacer_index = self.elements[root_spacer_index].u();
        let new_spacer = Link::Spacer(Spacer {
            dlink: root_spacer_index,
            ulink: bottom_spacer_index,
        });
        self.elements.push(new_spacer);
        // Patch old spacers
        //Old bottom dlink = new spacer
        self.elements[bottom_spacer_index].set_d(spacer_index);
        // Patch root ulink
        self.elements[root_spacer_index].set_u(spacer_index);

        // Add the entry to the hash table
        self.options.insert(spacer_index, option.to_vec());
        self.names.push(String::from(name));
        self.spacer_ids.insert(spacer_index, self.names.len() - 1);

        self
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
        match col {
            Link::Item(_) => {}
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

    /// Unlinks an item from the horizontally linked list
    fn unlink_item(&mut self, i: Index) {
        let l = self.elements[i].l();
        let r = self.elements[i].r();
        self.elements[l].set_r(r);
        self.elements[r].set_l(l);
    }

    /// Relinks an item into the horizontally linked list
    ///
    /// Must be done in the reverse order to unlinking
    fn relink_item(&mut self, i: Index) {
        let l = self.elements[i].l();
        let r = self.elements[i].r();
        self.elements[l].set_r(i);
        self.elements[r].set_l(i);
    }

    /// When selecting an option, this runs through all of the items it covers
    /// and unlinks those OptionElements vertically
    fn hide(&mut self, p: Index) -> Result<(), &'static str> {
        let mut q = p + 1;
        while q != p {
            let x = self.elements[q].top();
            let u = self.elements[q].u();
            let d = self.elements[q].d();

            match self.elements[q] {
                Link::Item(_) => return Err("Hide encountered and item"),
                Link::Spacer(_) => q = u,
                Link::OptionElement(_) => {
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

        match col {
            Link::Item(_) => {}
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

    /// Reverse of function [hide](crate::solver::Solver::hide)
    fn unhide(&mut self, p: Index) -> Result<(), &'static str> {
        let mut q = p - 1;
        while q != p {
            let x = self.elements[q].top();
            let u = self.elements[q].u();
            let d = self.elements[q].d();

            match self.elements[q] {
                Link::Item(_) => return Err("Hide encountered and item"),
                Link::Spacer(_) => q = d,
                Link::OptionElement(_) => {
                    self.elements[u].set_d(q);
                    self.elements[d].set_u(q);
                    self.elements[x].inc_l();
                }
            };
            q -= 1;
        }

        Ok(())
    }

    /// Implements algorithm X as a finite state machine
    #[allow(dead_code)]
    pub fn solve(&mut self) -> Option<Vec<String>> {
        // Follows stages of algorithm description in Fasc 5c, Knuth

        // The only ways to break this loop are to yield a solution via X2 or to
        // have exhausted all solutions via X8
        loop {
            match self.stage {
                Stage::X2 => {
                    if let Some(z) = self.x2() {
                        return Some(z);
                    }
                }
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
    }

    /// Returns a solution in a human-understandable form
    ///
    /// The solution vector `sol_vec` stores each of the OptionElements which
    /// were used to cover the items in the solution.  To turn this into
    /// something understandable we find the spacer to its right, and use this
    /// with a lookup table created earlier to map this to the names of options
    ///
    // TODO: Is it useful to have the double map? We don't used spacer_ids for
    //       anything else, so could condense it into a single HashMap
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
        if self.elements[0].r() == 0 || self.elements[0].r() >= self.optional {
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
        while idx != 0 && idx < self.optional {
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

            match &self.elements[p] {
                Link::Spacer(_) => {
                    // If a spacer, then hop up one link
                    p = self.elements[p].u();
                }
                op @ Link::OptionElement(_) => {
                    //                    println!("Covering X5: {}", j);
                    //                    println!("State:");
                    //                    println!("{}", self);
                    let j = op.top();

                    self.cover(j).unwrap();
                }
                Link::Item(x) => {
                    panic!("Trying an item {:?}", x);
                }
            };
            p += 1;
        }
        //        println!("--");

        self.l += 1;
        self.stage = Stage::X2;
        None
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
            }
        }
    }

    /// Takes in a non-item node and steps rightwards along `self.elements` the
    /// until a spacer is found, upon which the index is returned
    fn spacer_for(&self, x: Index) -> Index {
        let mut p = x;
        loop {
            match self.elements[p] {
                Link::Spacer(_) => return p,
                Link::OptionElement(_) => p += 1,
                Link::Item(_) => panic!("Somehow ended up on an item"),
            };
        }
    }

    /// Selects an option with the name `name` When setting up a general
    /// constraint solution, this is how to search for specific answers e.g. a
    /// Sudoku has all the constraints (items and options), and then the squares
    /// filled out in the specific problem need to be selected
    ///
    /// So for the problem
    ///
    /// ```text
    ///    i1  i2  i3
    /// o1  1   0   0
    /// o2  1   0   0
    /// o3  0   1   1
    /// ```
    /// Clearly *both* \[o1,o3\] and \[o2,o3\] are solutions, but if we select o1, then only one solution remains
    ///
    /// ```
    ///# use dlx_rs::solver::Solver;
    ///
    /// let mut s = Solver::new(3);
    ///
    /// s.add_option("o1", &[1])
    ///     .add_option("o2", &[1])
    ///     .add_option("o3", &[2, 3]);
    ///
    /// // First get all solutions
    /// let sols: Vec<Vec<String>> = s.clone().collect();
    /// assert_eq!( sols.len(), 2);
    /// assert_eq!( vec!["o3", "o1"], sols[0]);
    /// assert_eq!( vec!["o3", "o2"], sols[1]);
    ///
    ///
    /// // Now select o1 and get all solutions
    /// s.select("o1");
    /// assert_eq!( vec!["o3"], s.next().unwrap());
    /// ```
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
            match self.elements[p] {
                Link::OptionElement(_) => {
                    self.cover(self.elements[p].top()).unwrap();
                    p += 1;
                }
                Link::Spacer(_) => break,
                Link::Item(_) => break,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn spacer_for() {
        let mut s = Solver::new(4);
        s.add_option("o1", &[1, 2])
            .add_option("o2", &[2, 3])
            .add_option("o3", &[3, 4])
            .add_option("o4", &[1, 4]);

        // This creates a vec which looks like
        // [i0, i1, i2, i3, i4, s0
        //      x    x          s1
        //           x   x      s2
        //               x   x  s3
        //      x            x  s4]
        //

        let spacer_answers = HashMap::from([
            (6, 8),
            (7, 8),
            (8, 8),
            (9, 11),
            (10, 11),
            (11, 11),
            (12, 14),
            (13, 14),
            (14, 14),
            (15, 17),
            (16, 17),
            (17, 17),
        ]);

        for i in 6..=17 {
            assert_eq!(s.spacer_for(i), spacer_answers[&i]);
        }
    }
}
