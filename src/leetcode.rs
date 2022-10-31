use std::borrow::Borrow;
use std::collections;

pub fn main(){
    // let grid1=vec![
    //     vec!['0','1','0'],
    //     vec!['1','0','1'],
    //     vec!['0','1','0'],
    //   ];
    // let grid2=vec![
    //     vec!['1','1','1','1','0'],
    //     vec!['1','1','0','1','0'],
    //     vec!['1','1','0','0','0'],
    //     vec!['0','0','0','0','0'],
    //   ];
    // println!("{}",number_of_islands_recursive(grid2));

}

// 200
fn number_of_islands(grid: Vec<Vec<char>>)->i32{
    let m =grid.len();
    let n =grid[0].len();
    let mut stack=vec![];
    let mut num_land=0;
    let mut new_grid=grid.clone();

    for i in 0..m{
        for j in 0..n{
            if new_grid[i][j]=='1'{
                stack.push((i,j));
                while let Some(pos)=stack.pop(){
                    if pos.0<m&&pos.1<n&&new_grid[pos.0][pos.1]=='1'{
                        new_grid[pos.0][pos.1]='0';
                        if pos.1<n-1{
                            stack.push((pos.0,pos.1+1));
                        }
                        if pos.0<m-1{
                            stack.push((pos.0+1,pos.1));
                        }
                        if pos.1>0{
                            stack.push((pos.0,pos.1-1));
                        }
                        if pos.0>0{
                            stack.push((pos.0-1,pos.1));
                        }   
                    }
                }
                num_land+=1;
            }
        }
    }

    num_land
}

// 200
fn number_of_islands_recursive(grid: Vec<Vec<char>>)->i32{
    let m =grid.len();
    let n =grid[0].len();
    fn dfs(grid:&mut Vec<Vec<char>>,pos:(usize,usize)){
        if grid[pos.0][pos.1]=='0'{return}
        grid[pos.0][pos.1]='0';
        let (m,n)=(grid.len(),grid[0].len());
        if pos.1<n-1{
            dfs(grid, (pos.0,pos.1+1))
        }
        if pos.0<m-1{
            dfs(grid, (pos.0+1,pos.1))
        }
        if pos.1>0{
            dfs(grid, (pos.0,pos.1-1))
        }
        if pos.0>0{
            dfs(grid, (pos.0-1,pos.1))
        }   
    }
    let mut num_land=0;
    let mut new_grid=grid.clone();
    for i in 0..m{
        for j in 0..n{
            if new_grid[i][j]=='1'{
                num_land+=1;
                dfs(&mut new_grid, (i,j));
            }
        }
    }
    num_land
}


// 1
fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    use std::collections::HashMap;

    let mut m =HashMap::new();
    for (i,v) in nums.iter().enumerate(){
        match m.get(&(target-*v)){
            Some(&i2)=>return vec![i as i32,i2],
            None=>m.insert(*v, i as i32),
        };
    }
    vec![]
}

// 206
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
fn reverse_linked_list(head:Option<Box<ListNode>>)->Option<Box<ListNode>>{
    let  (mut curr,mut prev)=(head,None);
    while let Some(mut node)=curr{
        curr=node.next;
        node.next=prev;
        prev=Some(node)
    }
    prev
}



// 104
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
fn maximum_depth_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dpt(root: Option<Rc<RefCell<TreeNode>>>,current_depth:i32)->i32{
            if let Some(node)=root{
                let node=node.borrow();
                let left=dpt(node.left.clone(),current_depth+1);
                let right=dpt(node.right.clone(),current_depth+1);
                return std::cmp::max(left, right);
            }
            current_depth
        }
        let depth= dpt(root,0);
        depth
}


// 617
type Node=Rc<RefCell<TreeNode>>;
fn merge_tow_binary_tree(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    fn seach_and_merge(node1:&Option<Node>,node2:&Option<Node>)->Option<Node>{
        match (node1,node2){
            (Some(node1),Some(node2))=>{
                let (node1,node2)=(node1.borrow(),node2.borrow());
                let mut root=TreeNode::new(node1.val+node2.val);
                root.left=seach_and_merge(&node1.left, &node2.left);
                root.right=seach_and_merge(&node1.right, &node2.right);
                Some(Rc::new(RefCell::new(root)))
            }
            (None,Some(node2))=>Some(node2.clone()),
            (Some(node1),None)=>Some(node1.clone()),
            (None,None)=>None,
        }
    }
    seach_and_merge(&root1, &root2)
}

fn remove_duplicate_sorted_list2(head: Option<Box<ListNode>>)->Option<Box<ListNode>>{
    let prev=None;
    let node=head.clone();
    while let Some(node)=node{
        if let Some(next)=node.next{
            if node.val==next.val{
                prev=Some(node.clone());
                node=next;
            }
        }
    }
    prev

}

// 300
fn longest_increasing_subsequence(nums:Vec<i32>)->i32{
    let n =nums.len();
    let mut dp=vec![1;n];
    let mut answer=1;
    for i in 0..n{
        for j in 0..=i{
            if nums[i]>nums[j]&&dp[i]<dp[j]+1{
                dp[i]=dp[j]+1
            }
        }
        answer.max(dp[i]);
    }
    ;
}