//Instructions
// Manage a game player's High Score list.
//
// Your task is to build a high-score component of the classic Frogger game, one of the highest selling and addictive games of all time, and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.
//
// Consider retaining a reference to scores in the struct - copying is not necessary. You will require some lifetime annotations, though.

//指示
// 管理玩家的高分列表。
//
// 您的任务是构建经典的 Frogger 游戏的高分组件，这是有史以来销量最高且令人上瘾的游戏之一，也是街机时代的经典之作。你的任务是编写返回列表中最高分数、最后添加的分数和三个最高分数的方法。
//
// 考虑scores在结构中保留对的引用 - 不需要复制。不过，您将需要一些生命周期注释。

// [dependencies]
//
// [package]
// edition = "2021"
// name = "high-scores"
// version = "4.0.0"
#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    sort_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let s = scores.to_vec();
        let mut sort = s.clone();
        sort.sort_unstable();
        HighScores {
            scores: s,
            sort_scores: sort,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.sort_scores.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let len = self.sort_scores.len() as i32;
        let mut ret: Vec<u32> = Vec::new();
        if len == 0 {
            return ret;
        }
        for v in 1..=3 {
            if len - v < 0 {
                continue;
            }
            match self.sort_scores.get((len - v) as usize) {
                Some(x) => {
                    println!("push:{}", x);
                    ret.push(x.clone());
                }
                None => (),
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct HighScores2<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores2<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores2 { scores }
    }
    pub fn scores(&self) -> &[u32] {
        self.scores
    }
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut res_vec = self.scores.to_vec();
        res_vec.sort_unstable_by(|a, b| b.cmp(a));
        res_vec.truncate(3);
        res_vec
    }
}


#[test]
fn test_list_of_scores() {
    let expected = [30, 50, 20, 70];
    let high_scores = HighScores::new(&expected);
    assert_eq!(high_scores.scores(), &expected);
}

#[test]
#[ignore]
fn test_latest_score() {
    let high_scores = HighScores::new(&[100, 0, 90, 30]);
    assert_eq!(high_scores.latest(), Some(30));
}

#[test]
#[ignore]
fn test_latest_score_empty() {
    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.latest(), None);
}

#[test]
#[ignore]
fn test_personal_best() {
    let high_scores = HighScores::new(&[40, 100, 70]);
    assert_eq!(high_scores.personal_best(), Some(100));
}

#[test]
#[ignore]
fn test_personal_best_empty() {
    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.personal_best(), None);
}

#[test]
#[ignore]
fn test_personal_top_three() {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
    assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
}

#[test]
#[ignore]
fn test_personal_top_three_highest_to_lowest() {
    let high_scores = HighScores::new(&[20, 10, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
}

#[test]
#[ignore]
fn test_personal_top_three_with_tie() {
    let high_scores = HighScores::new(&[40, 20, 40, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
}

#[test]
#[ignore]
fn test_personal_top_three_with_less_than_three_scores() {
    let high_scores = HighScores::new(&[30, 70]);
    let s = high_scores.personal_top_three();
    println!("{:?}", s);
    assert_eq!(s, vec![70, 30]);
}

#[test]
#[ignore]
fn test_personal_top_three_only_one_score() {
    let high_scores = HighScores::new(&[40]);
    assert_eq!(high_scores.personal_top_three(), vec![40]);
}

#[test]
#[ignore]
fn test_personal_top_three_empty() {
    let high_scores = HighScores::new(&[]);
    assert!(high_scores.personal_top_three().is_empty());
}