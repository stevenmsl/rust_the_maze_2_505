use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
  - check question 490 for comments explaining
    why we visit the map the way we do here
  - the main difference between this one and
    question 490 is that we need to
    - exhaust the paths that can lead to the dest
    - report back the shortest path among them
  - we enhance the queue a bit to store not only
    the coordinates of the node but also the steps
    required to take from the start to the visiting
    node
*/

impl Solution {
  pub fn shortest_distance(
    start: &(usize, usize),
    dest: &(usize, usize),
    maze: &Vec<Vec<usize>>,
  ) -> i32 {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    Self::bfs(start, dest, maze, &mut visited)
  }

  fn bfs(
    node: &(usize, usize),
    dest: &(usize, usize),
    maze: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
  ) -> i32 {
    /*
      - right, left, down, up
    */
    let neighbors = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::from([(*node, 0)]);
    let rows = maze.len();
    let cols = maze[0].len();
    let (dest_x, dest_y) = dest;

    let mut min_distance = usize::MAX;

    while queue.len() > 0 {
      /*
        - use size to make sure we have
          visited all nodes at one level
          before moving on to the next
      */
      let size = queue.len();

      for _ in 0..size {
        if let Some(node_steps) = queue.pop_front() {
          let (node_x, node_y) = node_steps.0;
          /*
            - total steps to reach this node from the start
          */
          let steps = node_steps.1;
          if node_x == *dest_x && node_y == *dest_y {
            if steps < min_distance {
              min_distance = steps;
            }
          }
          for neighbor in &neighbors {
            let (x_move, y_move) = neighbor;
            let mut next_x_check = node_x as isize + x_move;
            let mut next_y_check = node_y as isize + y_move;

            let mut additional_steps: usize = 1;

            /* make sure the next move is within bounds */
            while next_x_check >= 0
              && next_x_check < rows as isize
              && next_y_check >= 0
              && next_y_check < cols as isize
              && maze[next_x_check as usize][next_y_check as usize] == 0
            {
              /*
                - keep moving to a partiuclar direction
                - either x_move or y_move one of them will
                  be zero; so we will be just moving along
                  either x or y direction whichever the move
                  is not zero
              */
              next_x_check += x_move;
              next_y_check += y_move;
              additional_steps += 1;
            }

            /*
              - we need to deduct one move back due to
                how we design the while loop
            */
            let next_x = (next_x_check - x_move) as usize;
            let next_y = (next_y_check - y_move) as usize;
            additional_steps -= 1;
            /*
              - to see if this node is our rightful candidate
                for the next round
            */

            if maze[next_x][next_y] == 0 && !visited[next_x][next_y] {
              visited[next_x][next_y] = true;
              /*
                - we also need to record how many steps it takes from
                  the start to reach this node
              */
              queue.push_back(((next_x, next_y), steps + additional_steps));
            }
          }
        }
      }
    }
    min_distance as i32
  }

  pub fn test_fixture_1() -> Vec<Vec<usize>> {
    vec![
      vec![0, 0, 1, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 1, 0],
      vec![1, 1, 0, 1, 1],
      vec![0, 0, 0, 0, 0],
    ]
  }

  pub fn test_fixture_2() -> Vec<Vec<usize>> {
    vec![
      vec![0, 0, 1, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 1, 0],
      vec![1, 1, 0, 1, 1],
      vec![0, 0, 0, 0, 0],
    ]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::shortest_distance(&(0, 4), &(4, 4), &Solution::test_fixture_1());
    assert_eq!(result, 12);
  }

  #[test]
  fn sample_2() {
    let result = Solution::shortest_distance(&(0, 4), &(3, 2), &Solution::test_fixture_2());
    assert_eq!(result, -1);
  }
}
