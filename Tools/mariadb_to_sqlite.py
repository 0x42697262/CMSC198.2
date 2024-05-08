import mysql.connector
import json
import sqlite3
from datetime import datetime
import argparse
from pathlib import Path

def select_mod(mod: str) -> int:
    match mod:
        case "EZ":
            return 1 << 1
        case "TD":
            return 1 << 2
        case "HD":
            return 1 << 3
        case "HR":
            return 1 << 4
        case "SD":
            return 1 << 5
        case "DT":
            return 1 << 6
        case "RL":
            return 1 << 7
        case "HT":
            return 1 << 8
        case "NC":
            return 1 << 9
        case "FL":
            return 1 << 10
        case "AT" | "AO":
            return 1 << 11
        case "SO":
            return 1 << 12
        case "AP":
            return 1 << 13
        case "PF":
            return 1 << 14
        case "CM":
            return 1 << 22
        case "TP":
            return 1 << 23
        case "SV2":
            return 1 << 29
        case _:
            return 0


def connect_to_database(host: str, user: str, password: str, db: str) -> mysql.connector.connection_cext.CMySQLConnection:
    conn = mysql.connector.connect(user=user, password=password, host=host, database=db)

    return conn

def fetch_players(conn: mysql.connector.connection_cext.CMySQLConnection) -> list:
    cur = conn.cursor()
    cur.execute("SELECT user_id FROM osu_user_stats")
    players = cur.fetchall()

    return players

def insert_to_sqlite_db(row, cur):
    user_id         = row[0]
    player_pp       = row[1]
    date_unix       = row[2]
    combo           = row[3]
    score           = row[4]
    acc             = row[5]
    pp              = row[6]
    map_combo       = row[7]
    duration        = row[8]
    stars           = row[9]
    hp_drain        = row[10]
    cs              = row[11]
    od              = row[12]
    ar              = row[13]
    bpm             = row[14]
    circle          = row[15]
    slider          = row[16]
    spinner         = row[17]
    data            = json.loads(row[-1].decode('utf-8'))

    if "NF" in data:
        return

    stats = data['statistics']
    mods = [mod['acronym'] for mod in data['mods']]
    mod = 0
    for m in mods:
        mod += select_mod(m)

    # this will be reallly slow ride
    great: int  = stats.get('great', 0)
    ok: int     = stats.get('ok', 0)
    meh: int    = stats.get('meh', 0)
    miss: int   = stats.get('miss', 0)

    cur.execute("""
        INSERT INTO scores
        VALUES (
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?
            )
        """, (user_id, player_pp, date_unix, combo, score, acc, great, ok, meh, miss, pp, mod, map_combo, duration, stars, hp_drain, cs, od ,ar, bpm, circle, slider, spinner))


def process_player(conn: mysql.connector.connection_cext.CMySQLConnection, players: list, output: Path):
    cur = conn.cursor()
    counter = 1
    total_players = len(players)

    for player in players:
        id = player[0]
        time = datetime.now().strftime("%H:%M:%S")
        print(f"{time} Dumping player    {id:<10} ({counter}/{total_players})")

        cur.execute(f"""
            SELECT osu_user_stats.user_id,
                   osu_user_stats.rank_score,
                   scores.unix_updated_at,
                   scores.max_combo,
                   scores.total_score,
                   scores.accuracy,
                   scores.pp,
                   osu_beatmaps.max_combo,
                   osu_beatmaps.hit_length,
                   osu_beatmaps.difficultyrating,
                   osu_beatmaps.diff_drain,
                   osu_beatmaps.diff_size,
                   osu_beatmaps.diff_overall,
                   osu_beatmaps.diff_approach,
                   osu_beatmaps.bpm,
                   osu_beatmaps.countNormal,
                   osu_beatmaps.countSlider,
                   osu_beatmaps.countSpinner,
                   data
            FROM osu_user_stats
            JOIN scores ON osu_user_stats.user_id = scores.user_id
            JOIN osu_beatmaps ON scores.beatmap_id = osu_beatmaps.beatmap_id
            WHERE osu_user_stats.user_id = {id} ORDER BY scores.unix_updated_at ASC
        """)

        player_db: Path = output / f"{id}.db"
        sqlite_conn = sqlite3.connect(player_db)
        sqlite_cursor = sqlite_conn.cursor()
        sqlite_cursor.execute("""
            CREATE TABLE IF NOT EXISTS scores (
                user_id INTEGER,
                player_pp REAL,
                date_unix INTEGER,
                combo INTEGER,
                total_score INTEGER,
                accuracy REAL,
                great INTEGER,
                ok INTEGER,
                meh INTEGER,
                miss INTEGER,
                pp REAL,
                mod INTEGER,
                map_combo INTEGER,
                duration INTEGER,
                stars REAL,
                hp_drain INTEGER,
                circle_size INTEGER,
                overall_difficulty INTEGER,
                approach_rate INTEGER,
                bpm INTEGER,
                countNormal INTEGER,
                countSlider INTEGER,
                countSpinner INTEGER
            )
        """)

        while True:
            row = cur.fetchone()
            if not row:
                break
            insert_to_sqlite_db(row, sqlite_cursor)

        sqlite_conn.commit()
        sqlite_conn.close()
        counter += 1


def main():
    parser = argparse.ArgumentParser(
        description="Dumps osu! database mysql from server to multiple player sqlite databases",
    )
    parser.add_argument('-u', '--user',
                        type=str,
                        default="root",
                        help="mysql server user")
    parser.add_argument('-p', '--password',
                        type=str,
                        default="cmsc198",
                        help="mysql server password")
    parser.add_argument('-s', '--server',
                        type=str,
                        default="172.17.0.1",
                        help="mysql server host address")
    parser.add_argument('-d', '--database',
                        type=str,
                        default="osu",
                        help="mysql server database")
    parser.add_argument('-o', '--output',
                        type=Path,
                        default="./../Data/players/",
                        help="output directory for sqlite3 databases")
    args = parser.parse_args()

    USER: str = args.user
    PASSWORD: str = args.password
    HOST: str = args.server
    DB: str = args.database
    OUTPUT_DIR: Path = args.output

    if not OUTPUT_DIR.exists():
        print(f"Output directory does not exist: {OUTPUT_DIR}")
        return 1
    if OUTPUT_DIR.is_file():
        print(f"Output directory is a file: {OUTPUT_DIR}")
        return 1

    conn = connect_to_database(HOST, USER, PASSWORD, DB)

    players = fetch_players(conn)
    process_player(conn, players, OUTPUT_DIR)

    conn.close()

    print("Dumping of players data complete.")
    

if __name__ == "__main__":
    main()
