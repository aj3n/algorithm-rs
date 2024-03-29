/**
 * [1235] Maximum Profit in Job Scheduling
 *
 * We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].
 * You're given the startTime, endTime and profit arrays, return the maximum profit you can take such that there are no two jobs in the subset with overlapping time range.
 * If you choose a job that ends at time X you will be able to start another job that starts at time X.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/10/sample1_1584.png" style="width: 380px; height: 154px;" />
 *
 * Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
 * Output: 120
 * Explanation: The subset chosen is the first and fourth job.
 * Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/10/sample22_1584.png" style="width: 600px; height: 112px;" />
 *
 * Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
 * Output: 150
 * Explanation: The subset chosen is the first, fourth and fifth job.
 * Profit obtained 150 = 20 + 70 + 60.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/10/sample3_1584.png" style="width: 400px; height: 112px;" />
 *
 * Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
 * Output: 6
 *
 *  
 * Constraints:
 *
 *     1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4
 *     1 <= startTime[i] < endTime[i] <= 10^9
 *     1 <= profit[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
		let mut jobs = start_time
			.into_iter()
			.zip(end_time)
			.zip(profit)
			.collect::<Vec<_>>();
		jobs.sort_unstable();
		Self::solve(&jobs, &mut vec![-1; jobs.len() + 1])
	}
	fn solve(jobs: &[((i32, i32), i32)], cache: &mut [i32]) -> i32 {
		if cache[jobs.len()] >= 0 {
			return cache[jobs.len()];
		}
		match jobs {
			[job] => job.1,
			[] => 0,
			[((_, end), profit), rest @ ..] => {
				let mut ans = Self::solve(rest, cache);
				let next = rest.partition_point(|v| v.0 .0 < *end);
				ans = ans.max(profit + Self::solve(&rest[next..], cache));
				cache[jobs.len()] = ans;
				ans
			}
		}
	}
	pub fn job_scheduling1(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
		let mut jobs = start_time
			.into_iter()
			.zip(end_time)
			.zip(profit)
			.collect::<Vec<_>>();
		jobs.sort_unstable();
		let mut max_profit = std::collections::BinaryHeap::<(i32, i32)>::new();
		let mut profit_sofar = 0;
		for ((start, end), profit) in jobs {
			while let Some(top) = max_profit.peek() {
				if -top.0 <= start {
					profit_sofar = profit_sofar.max(top.1);
					max_profit.pop();
				} else {
					break;
				}
			}
			max_profit.push((-end, profit_sofar + profit));
		}
		max_profit
			.into_iter()
			.map(|v| v.1)
			.max()
			.unwrap()
			.max(profit_sofar)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1235() {
		assert_eq!(
			Solution::job_scheduling(
				vec![4, 2, 4, 8, 2],
				vec![5, 5, 5, 10, 8],
				vec![1, 2, 8, 10, 4]
			),
			18
		);
		assert_eq!(
			Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
			120
		);
		assert_eq!(
			Solution::job_scheduling(
				vec![1, 2, 3, 4, 6],
				vec![3, 5, 10, 6, 9],
				vec![20, 20, 100, 70, 60]
			),
			150
		);
		assert_eq!(
			Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
			6
		);
	}
}
