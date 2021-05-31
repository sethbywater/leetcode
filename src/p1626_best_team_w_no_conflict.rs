//! # 1626. Best team with no conflicts
//! 
//! You are the manager of a basketball team. For the upcoming tournament, 
//! you want to choose the team with the highest overall score. The score 
//! of the team is the ~~sum~~ of scores of all the players in the team.
//! 
//! However, the basketball team is not allowed to have ~~conflicts.~~ A conflict 
//! exists if a younger player has a ~~strictly higher~~ score than an older player. 
//! A conflict does not occur between players of the same age.
//! 
//! Given two lists, scores and ages, where each scores[i] and ages[i] represents 
//! the score and age of the ith player, respectively, return ~the highest overall 
//! score of all possible basketball teams.~

pub fn best_team_score(mut scores: Vec<i32>, mut ages: Vec<i32>) -> i32 {
	if scores.len() == 0 { return 0 }
	sort_age_score(&mut scores, &mut ages);
	let mut best = scores.clone();
	let mut hi = 0;
	for i in 0..scores.len() {
		for j in 0..i {
			if scores[j] <= scores[i] { best[i] = best[i].max(best[j] + scores[i]) }
		}
		hi = hi.max(best[i])
	}
	hi
}


/// Sorts by age, then by score within each age
fn sort_age_score(scores: &mut Vec<i32>, ages: &mut Vec<i32>) {
	for i in 0..ages.len() {
		let mut j = i;
		while j > 0 && (ages[j] < ages[j-1] || (ages[j] == ages[j-1] && scores[j] < scores[j-1])) {
			ages.swap(j, j-1);
			scores.swap(j, j-1);
			j -= 1
		}
	}
}