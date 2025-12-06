use eframe::egui;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct WordEntry {
    id: u32,
    word: String,
    category: String,
    difficulty: u32,
}

fn load_words_from_csv(path: &str) -> Result<Vec<WordEntry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut words = Vec::new();
    
    for result in rdr.deserialize() {
        let record: WordEntry = result?;
        words.push(record);
    }
    
    Ok(words)
}

fn shuffle_japanese_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut rng = thread_rng();
    
    // Keep shuffling until the result is different from the original
    loop {
        chars.shuffle(&mut rng);
        let shuffled: String = chars.iter().collect();
        if shuffled != input {
            return shuffled;
        }
    }
}

struct AnagramApp {
    words: Vec<WordEntry>,
    current_word: Option<String>,
    shuffled_word: String,
    user_input: String,
    message: String,
    message_color: egui::Color32,
}

impl Default for AnagramApp {
    fn default() -> Self {
        let words = load_words_from_csv("assets/japanese_dictionary.csv")
            .expect("Failed to load dictionary");
        
        Self {
            words,
            current_word: None,
            shuffled_word: String::new(),
            user_input: String::new(),
            message: "「新しいゲーム」をクリックして開始".to_string(),
            message_color: egui::Color32::GRAY,
        }
    }
}

impl AnagramApp {
    fn start_new_game(&mut self) {
        let mut rng = thread_rng();
        if let Some(entry) = self.words.choose(&mut rng) {
            let word = entry.word.trim();
            self.current_word = Some(word.to_string());
            self.shuffled_word = shuffle_japanese_string(word);
            self.user_input.clear();
            self.message = "シャッフルされた文字を並び替えて元の単語を当ててください！".to_string();
            self.message_color = egui::Color32::GRAY;
        }
    }
    
    fn check_answer(&mut self) {
        if let Some(ref correct_answer) = self.current_word {
            if self.user_input.trim() == correct_answer {
                self.message = "正解！🎉".to_string();
                self.message_color = egui::Color32::GREEN;
            } else {
                self.message = format!("不正解... 正解は「{}」でした", correct_answer);
                self.message_color = egui::Color32::RED;
            }
        }
    }
    
    fn give_up(&mut self) {
        if let Some(ref correct_answer) = self.current_word {
            self.message = format!("正解は「{}」でした", correct_answer);
            self.message_color = egui::Color32::YELLOW;
        }
    }
}

impl eframe::App for AnagramApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔤 アナグラムゲーム");
            ui.add_space(20.0);
            
            // New Game Button
            if ui.button("🎮 新しいゲーム").clicked() {
                self.start_new_game();
            }
            
            ui.add_space(20.0);
            
            // Display shuffled word
            if self.current_word.is_some() {
                ui.horizontal(|ui| {
                    ui.label("シャッフルされた文字:");
                    ui.heading(&self.shuffled_word);
                });
                
                ui.add_space(15.0);
                
                // Input field
                ui.horizontal(|ui| {
                    ui.label("あなたの答え:");
                    let response = ui.text_edit_singleline(&mut self.user_input);
                    
                    // Submit on Enter key
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.check_answer();
                    }
                });
                
                ui.add_space(10.0);
                
                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("✓ 答え合わせ").clicked() {
                        self.check_answer();
                    }
                    
                    if ui.button("🏳 ギブアップ").clicked() {
                        self.give_up();
                    }
                });
            }
            
            ui.add_space(20.0);
            
            // Message display
            ui.colored_label(self.message_color, &self.message);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 350.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Anagram Game"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Anagram Game",
        options,
        Box::new(|cc| {
            // Load Japanese fonts
            let mut fonts = egui::FontDefinitions::default();
            
            // Add Japanese font data (using system fonts or embedded fonts)
            // For Windows, we'll use the system's Japanese fonts
            fonts.font_data.insert(
                "japanese".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/NotoSansJP-Regular.ttf")),
            );
            
            // Set Japanese font as highest priority for proportional text
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "japanese".to_owned());
            
            // Set Japanese font for monospace as well
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("japanese".to_owned());
            
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(AnagramApp::default()))
        }),
    )
}
