import mysql.connector
import sqlite3
from datetime import datetime

maria_conn = mysql.connector.connect(user='root', password='cmsc198', host='172.17.0.1', database='osu')
maria_cursor = maria_conn.cursor()

maria_cursor.execute("SELECT user_id FROM osu_user_stats")
players = maria_cursor.fetchall()
total_players = len(players)

counter = 1
for player in players:
    id = player[0]
    time = datetime.now().strftime("%H:%M:%S")
    print(f"{time} Dumping player    {id:<10} ({counter}/{total_players})")

    maria_cursor.execute(f"""
    SELECT osu_user_stats.user_id
      , osu_user_stats.rank_score AS "player_pp"
      , scores.unix_updated_at AS "date_unix"
      , scores.max_combo AS "combo"
      , osu_beatmaps.max_combo AS "max_combo"
      , scores.total_score
      , scores.accuracy
      , scores.pp
      , osu_beatmaps.difficultyrating AS "stars"
      , osu_beatmaps.countNormal
      , osu_beatmaps.countSlider
      , osu_beatmaps.countSpinner
      , data
    FROM osu_user_stats
    JOIN scores ON osu_user_stats.user_id = osu_user_stats.user_id
    JOIN osu_beatmaps ON scores.beatmap_id = osu_beatmaps.beatmap_id
    WHERE user_id = {id}
    """)
    rows = maria_cursor.fetchall()
    sqlite_conn = sqlite3.connect(f"./../Data/players/{id}.db")
    sqlite_cursor = sqlite_conn.cursor()
    sqlite_cursor.execute("""
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
        data TEXT
    )
    """)

    sqlite_cursor.executemany("""
    INSERT INTO scores
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    """, rows)
    sqlite_conn.commit()
    sqlite_conn.close()

    counter += 1

maria_conn.close()
