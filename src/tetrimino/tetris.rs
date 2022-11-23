mod tetris {
    use crate::tetrimino::create_tetriminos::create_tetriminos::Tetrimino;

    struct Tetris {
         game_map: Vec<Vec<u8>>,
         current_level: u32,
         score: u32,
         nb_lines: u32,
         current_piece: Option<Tetrimino>,
     }

    impl Tetris {
        fn new() -> Tetris {
            let mut game_map = Vec::new();

            for _ in 0..16 {
                game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) ;
            }
            Tetris {
                game_map,
                current_level: 1,
                score: 0,
                nb_lines: 0,
                current_piece: None
            }
        }

        fn check_lines(&mut self) {
            let mut y = 0;

            while y < self.game_map.len() {
                let mut complete = true;

                for x in &self.game_map[y] {
                    if *x == 0 {
                        complete = false;
                        break
                    }
                }
                if complete == true {
                    self.game_map.remove(y);
                    y -= 1;
                    // increase the number if seld.lines
                }
                y += 1;
            }
            while self.game_map.len() < 16 {
                self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            }
        }
    }
}
