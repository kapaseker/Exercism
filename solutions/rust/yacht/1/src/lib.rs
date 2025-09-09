#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        // 数字类别：计算对应数字的总和
        Category::Ones => dice.iter().filter(|&&d| d == 1).count() as u8 * 1,
        Category::Twos => dice.iter().filter(|&&d| d == 2).count() as u8 * 2,
        Category::Threes => dice.iter().filter(|&&d| d == 3).count() as u8 * 3,
        Category::Fours => dice.iter().filter(|&&d| d == 4).count() as u8 * 4,
        Category::Fives => dice.iter().filter(|&&d| d == 5).count() as u8 * 5,
        Category::Sixes => dice.iter().filter(|&&d| d == 6).count() as u8 * 6,

        // 满堂彩 (Full House)：三个相同数字加两个相同数字
        Category::FullHouse => {
            let mut counts = [0; 7]; // 索引0-6，只使用1-6
            for &die in &dice {
                counts[die as usize] += 1;
            }

            let mut has_three = false;
            let mut has_two = false;

            for count in counts.iter() {
                if *count == 3 {
                    has_three = true;
                } else if *count == 2 {
                    has_two = true;
                }
            }

            if has_three && has_two {
                dice.iter().sum()
            } else {
                0
            }
        }

        // 四条 (Four of a Kind)：四个相同数字
        Category::FourOfAKind => {
            let mut counts = [0; 7];
            for &die in &dice {
                counts[die as usize] += 1;
            }

            for (value, &count) in counts.iter().enumerate() {
                if count >= 4 {
                    return (value as u8) * 4;
                }
            }
            0
        }

        // 小顺 (Little Straight)：1-2-3-4-5
        Category::LittleStraight => {
            let mut sorted_dice = dice;
            sorted_dice.sort();
            if sorted_dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }

        // 大顺 (Big Straight)：2-3-4-5-6
        Category::BigStraight => {
            let mut sorted_dice = dice;
            sorted_dice.sort();
            if sorted_dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }

        // 机会 (Choice)：所有骰子点数之和
        Category::Choice => dice.iter().sum(),

        // 游艇 (Yacht)：五个骰子点数相同
        Category::Yacht => {
            if dice.iter().all(|&d| d == dice[0]) {
                50
            } else {
                0
            }
        }
    }
}
