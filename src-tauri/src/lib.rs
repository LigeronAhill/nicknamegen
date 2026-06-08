// ==================== ВСПОМОГАТЕЛЬНЫЕ МАССИВЫ ДЛЯ РАЗНЫХ РАС ====================

use rand::RngExt;
// ----- Эльфы -----
// Английский
const ELF_EN_START: [&str; 10] = [
    "Ae", "Fae", "Lael", "Cael", "Syl", "El", "Thae", "Lian", "Rael", "Aerin",
];
const ELF_EN_MID: [&str; 10] = ["", "la", "ri", "va", "thy", "li", "na", "del", "wen", "nor"];
const ELF_EN_END_MALE: [&str; 10] = [
    "rion", "thor", "las", "nor", "dil", "dan", "mir", "thal", "ion", "os",
];
const ELF_EN_END_FEMALE: [&str; 10] = [
    "riel", "wyn", "thas", "lora", "ndila", "wen", "ariel", "eth", "iel", "ara",
];

// Русский (кириллица)
const ELF_RU_START: [&str; 10] = [
    "Аэ",
    "Фаэ",
    "Лаэль",
    "Каэ",
    "Силь",
    "Эль",
    "Таэ",
    "Лиан",
    "Раэль",
    "Аэрин",
];
const ELF_RU_MID: [&str; 10] = ["", "ла", "ри", "ва", "ти", "ли", "на", "дэль", "вэн", "нор"];
const ELF_RU_END_MALE: [&str; 10] = [
    "рион", "тор", "лас", "нор", "диль", "дан", "мир", "таль", "ион", "ос",
];
const ELF_RU_END_FEMALE: [&str; 10] = [
    "риэль",
    "вин",
    "тас",
    "лора",
    "ндила",
    "вэн",
    "ариэль",
    "эт",
    "иэль",
    "ара",
];

// ----- Люди -----
// Английский
const HUMAN_EN_START: [&str; 10] = [
    "Ar", "Ber", "Car", "Der", "Er", "Fer", "Gar", "Har", "Lar", "Mer",
];
const HUMAN_EN_MID: [&str; 10] = ["", "an", "en", "in", "on", "er", "or", "ur", "ir", "ar"];
const HUMAN_EN_END_MALE: [&str; 10] = ["d", "n", "r", "s", "th", "k", "l", "m", "t", "x"];
const HUMAN_EN_END_FEMALE: [&str; 10] = [
    "a", "ia", "na", "ra", "ssa", "ella", "ine", "ette", "ara", "iana",
];

// Русский
const HUMAN_RU_START: [&str; 10] = [
    "Ар", "Бер", "Кар", "Дер", "Эр", "Фер", "Гар", "Хар", "Лар", "Мер",
];
const HUMAN_RU_MID: [&str; 10] = ["", "ан", "ен", "ин", "он", "эр", "ор", "ур", "ир", "ар"];
const HUMAN_RU_END_MALE: [&str; 10] = ["д", "н", "р", "с", "т", "к", "л", "м", "ть", "кс"];
const HUMAN_RU_END_FEMALE: [&str; 10] = [
    "а", "ия", "на", "ра", "сса", "элла", "ина", "етта", "ара", "иана",
];

// ----- Орки -----
// Английский
const ORC_EN_START: [&str; 10] = [
    "Gr", "Mog", "Rok", "Dur", "Kra", "Thok", "Grom", "Zog", "Bur", "Skar",
];
const ORC_EN_MID: [&str; 10] = ["", "a", "o", "u", "ag", "e", "i", "or", "ur", "ak"];
const ORC_EN_END_MALE: [&str; 10] = ["ok", "og", "ar", "ash", "uk", "ak", "ol", "ur", "oth", "um"];
const ORC_EN_END_FEMALE: [&str; 10] = ["a", "ka", "ra", "sha", "za", "ga", "ta", "na", "la", "ma"];

// Русский
const ORC_RU_START: [&str; 10] = [
    "Гр", "Мог", "Рок", "Дур", "Кра", "Ток", "Гром", "Зог", "Бур", "Скар",
];
const ORC_RU_MID: [&str; 10] = ["", "а", "о", "у", "аг", "е", "и", "ор", "ур", "ак"];
const ORC_RU_END_MALE: [&str; 10] = ["ок", "ог", "ар", "аш", "ук", "ак", "ол", "ур", "от", "ум"];
const ORC_RU_END_FEMALE: [&str; 10] = ["а", "ка", "ра", "ша", "за", "га", "та", "на", "ла", "ма"];

// ----- Демоны -----
// Английский
const DEMON_EN_START: [&str; 10] = [
    "Mal", "Bel", "Az", "Mor", "Xar", "Zor", "Thal", "Vul", "Kor", "Nix",
];
const DEMON_EN_MID: [&str; 10] = ["", "a", "e", "i", "o", "u", "ae", "ei", "ou", "au"];
const DEMON_EN_END_MALE: [&str; 10] =
    ["oth", "ax", "on", "us", "ath", "an", "ar", "or", "um", "in"];
const DEMON_EN_END_FEMALE: [&str; 10] = [
    "a", "ia", "ara", "essa", "ix", "ine", "ira", "ora", "ula", "ys",
];

// Русский
const DEMON_RU_START: [&str; 10] = [
    "Мал", "Бел", "Аз", "Мор", "Ксар", "Зор", "Тал", "Вул", "Кор", "Никс",
];
const DEMON_RU_MID: [&str; 10] = ["", "а", "е", "и", "о", "у", "аэ", "эй", "оу", "ау"];
const DEMON_RU_END_MALE: [&str; 10] = ["от", "акс", "он", "ус", "ат", "ан", "ар", "ор", "ум", "ин"];
const DEMON_RU_END_FEMALE: [&str; 10] = [
    "а", "ия", "ара", "есса", "икс", "ина", "ира", "ора", "ула", "ис",
];

// ==================== ГЛАВНАЯ ФУНКЦИЯ ГЕНЕРАЦИИ ====================

#[tauri::command]
fn generate_nickname(sex: &str, race: &str, language: &str) -> String {
    let mut rng = rand::rng();

    // Определяем, male или female
    let is_male = match sex {
        "female" => false,
        _ => true,
    };

    // Выбираем наборы слогов в зависимости от расы и языка
    let (start_arr, mid_arr, end_arr) = match (race, language) {
        // Эльфы
        ("elf", "ru") => (
            ELF_RU_START.as_slice(),
            ELF_RU_MID.as_slice(),
            if is_male {
                ELF_RU_END_MALE.as_slice()
            } else {
                ELF_RU_END_FEMALE.as_slice()
            },
        ),
        ("elf", _) => (
            ELF_EN_START.as_slice(),
            ELF_EN_MID.as_slice(),
            if is_male {
                ELF_EN_END_MALE.as_slice()
            } else {
                ELF_EN_END_FEMALE.as_slice()
            },
        ),
        // Люди
        ("human", "ru") => (
            HUMAN_RU_START.as_slice(),
            HUMAN_RU_MID.as_slice(),
            if is_male {
                HUMAN_RU_END_MALE.as_slice()
            } else {
                HUMAN_RU_END_FEMALE.as_slice()
            },
        ),
        // Орки
        ("orc", "ru") => (
            ORC_RU_START.as_slice(),
            ORC_RU_MID.as_slice(),
            if is_male {
                ORC_RU_END_MALE.as_slice()
            } else {
                ORC_RU_END_FEMALE.as_slice()
            },
        ),
        ("orc", _) => (
            ORC_EN_START.as_slice(),
            ORC_EN_MID.as_slice(),
            if is_male {
                ORC_EN_END_MALE.as_slice()
            } else {
                ORC_EN_END_FEMALE.as_slice()
            },
        ),
        // Демоны
        ("demon", "ru") => (
            DEMON_RU_START.as_slice(),
            DEMON_RU_MID.as_slice(),
            if is_male {
                DEMON_RU_END_MALE.as_slice()
            } else {
                DEMON_RU_END_FEMALE.as_slice()
            },
        ),
        ("demon", _) => (
            DEMON_EN_START.as_slice(),
            DEMON_EN_MID.as_slice(),
            if is_male {
                DEMON_EN_END_MALE.as_slice()
            } else {
                DEMON_EN_END_FEMALE.as_slice()
            },
        ),
        _ => (
            HUMAN_EN_START.as_slice(),
            HUMAN_EN_MID.as_slice(),
            if is_male {
                HUMAN_EN_END_MALE.as_slice()
            } else {
                HUMAN_EN_END_FEMALE.as_slice()
            },
        ),
    };

    // Генерируем индексы
    let start_idx = rng.random_range(0..start_arr.len());
    let mid_idx = rng.random_range(0..mid_arr.len());
    let end_idx = rng.random_range(0..end_arr.len());

    // Собираем никнейм
    format!(
        "{}{}{}",
        start_arr[start_idx], mid_arr[mid_idx], end_arr[end_idx]
    )
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_nickname])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
