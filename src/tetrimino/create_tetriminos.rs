//! Create Tetriminos Pieces
//!
//! This crate contains several function and impl for tetriminos creation
//! It verify too if pieces can be used and rotate in game

///This module manage tetrimino creation
pub mod create_tetriminos {

    type Piece = Vec<Vec<u8>>;
    type States = Vec<Piece>;

    pub struct TetriminoI;
    pub struct TetriminoJ;
    pub struct TetriminoL;
    pub struct TetriminoO;
    pub struct TetriminoS;
    pub struct TetriminoT;
    pub struct TetriminoZ;

    /// Tetriminos structure
    pub struct Tetrimino {
        pub states: States,
        pub x: isize,
        pub y: usize,
        pub current_state: u8,
    }

    /// Trait used for Tetriminos generation
    pub trait TetriminoGenerator {
        fn new() -> Tetrimino;
    }

    impl Tetrimino {
        /// Rotate a tetriminos
        pub fn rotate(&mut self, game_map: &[Vec<u8>]) {
            let mut tmp_state = self.current_state + 1;
            if tmp_state as usize >= self.states.len() {
                tmp_state = 0;
            }
            let x_pos = [0, -1, 1, -2, 2, -3];
            for x in x_pos.iter() {
                if self.test_position(game_map, tmp_state as usize, self.x + x, self.y) == true {
                    self.current_state = tmp_state;
                    self.x += *x;
                    break;
                }
            }
        }

        /// Test if position for a tetriminos is possible or not
        fn test_position(
            &self,
            game_map: &[Vec<u8>],
            tmp_state: usize,
            x: isize,
            y: usize,
        ) -> bool {
            for shift_y in 0..4 {
                for shift_x in 0..4 {
                    let x = x + shift_x;
                    if self.states[tmp_state][shift_y][shift_x as usize] != 0
                        && (y + shift_y >= game_map.len()
                            || x < 0
                            || x as usize >= game_map[y + shift_y].len()
                            || game_map[y + shift_y][x as usize] != 0)
                    {
                        return false;
                    }
                }
            }
            return true;
        }

        pub fn change_position(
            &mut self,
            game_map: &[Vec<u8>],
            new_x: isize,
            new_y: usize,
        ) -> bool {
            if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
                self.x = new_x as isize;
                self.y = new_y;
                true
            } else {
                false
            }
        }

        pub fn test_current_position(&mut self, game_map: &[Vec<u8>]) -> bool {
            self.test_position(game_map, self.current_state as usize, self.x, self.y)
        }
    }

    impl TetriminoGenerator for TetriminoI {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![
                    vec![
                        vec![1, 1, 1, 1],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 1, 0, 0],
                        vec![0, 1, 0, 0],
                        vec![0, 1, 0, 0],
                        vec![0, 1, 0, 0],
                    ],
                ],
                x: 4,
                y: 0,
                current_state: 0,
            }
        }
    }

    impl TetriminoGenerator for TetriminoJ {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![
                    vec![
                        vec![2, 2, 2, 0],
                        vec![2, 0, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![2, 2, 0, 0],
                        vec![0, 2, 0, 0],
                        vec![0, 2, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 0, 2, 0],
                        vec![2, 2, 2, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![2, 0, 0, 0],
                        vec![2, 0, 0, 0],
                        vec![2, 2, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                ],
                x: 4,
                y: 0,
                current_state: 0,
            }
        }
    }

    impl TetriminoGenerator for TetriminoL {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![
                    vec![
                        vec![3, 3, 3, 0],
                        vec![0, 0, 3, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 3, 0, 0],
                        vec![0, 3, 0, 0],
                        vec![3, 3, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![3, 0, 0, 0],
                        vec![3, 3, 3, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![3, 3, 0, 0],
                        vec![3, 0, 0, 0],
                        vec![3, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                ],
                x: 4,
                y: 0,
                current_state: 0,
            }
        }
    }

    impl TetriminoGenerator for TetriminoO {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![vec![
                    vec![4, 4, 0, 0],
                    vec![4, 4, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ]],
                x: 5,
                y: 0,
                current_state: 0,
            }
        }
    }

    impl TetriminoGenerator for TetriminoS {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![
                    vec![
                        vec![0, 5, 5, 0],
                        vec![5, 5, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 5, 0, 0],
                        vec![0, 5, 5, 0],
                        vec![0, 0, 5, 0],
                        vec![0, 0, 0, 0],
                    ],
                ],
                x: 4,
                y: 0,
                current_state: 0,
            }
        }
    }

    impl TetriminoGenerator for TetriminoZ {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![
                    vec![
                        vec![6, 6, 0, 0],
                        vec![0, 6, 6, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 6, 0, 0],
                        vec![6, 6, 0, 0],
                        vec![6, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                ],
                x: 4,
                y: 0,
                current_state: 0,
            }
        }
    }

    impl TetriminoGenerator for TetriminoT {
        fn new() -> Tetrimino {
            Tetrimino {
                states: vec![
                    vec![
                        vec![7, 7, 7, 0],
                        vec![0, 7, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 7, 0, 0],
                        vec![7, 7, 0, 0],
                        vec![0, 7, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 7, 0, 0],
                        vec![7, 7, 7, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 7, 0, 0],
                        vec![0, 7, 7, 0],
                        vec![0, 7, 0, 0],
                        vec![0, 0, 0, 0],
                    ],
                ],
                x: 4,
                y: 0,
                current_state: 0,
            }
        }
    }
}
