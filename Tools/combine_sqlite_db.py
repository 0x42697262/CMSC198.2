import sqlite3
from pathlib import Path
from tqdm import tqdm


SCORES_DIR: Path = Path("../Data/sample/")
# OUTPUT_DB: Path = Path("sample.db")
OUTPUT_DB: Path = Path("../Data/sample.db")

if not SCORES_DIR.exists() or SCORES_DIR.is_file():
    import os
    os.quit()

output_conn = sqlite3.connect(OUTPUT_DB)
output_cursor = output_conn.cursor()

output_cursor.execute("""
    CREATE TABLE IF NOT EXISTS scores (
        user_id INTEGER,
        player_pp REAL,
        date_unix INTEGER,
        combo INTEGER,
        max_combo_beatmap INTEGER,
        total_score INTEGER,
        accuracy REAL,
        pp REAL,
        stars REAL,
        countNormal INTEGER,
        countSlider INTEGER,
        countSpinner INTEGER,
        great INTEGER,
        ok INTEGER,
        meh INTEGER,
        miss INTEGER
    )
""")

output_conn.commit()

database_files = [file for file in SCORES_DIR.iterdir() if file.is_file()]
for index, db in enumerate(tqdm(database_files, desc="Combining databases", unit="file")):
    conn = sqlite3.connect(db)
    cursor = conn.cursor()

    cursor.execute("SELECT * FROM scores")
    output = cursor.fetchall()

    for row in output:
        output_cursor.execute("INSERT INTO scores VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", row)
    cursor.close()

output_conn.commit()
output_cursor.close()

print(f"Merged {len(database_files)} databases.")
