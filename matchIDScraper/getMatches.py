import json
import os
from urllib.request import Request, urlopen

import psycopg2 as psycopg2
import time

APIKEY = os.environ.get("APIKEY")
if not APIKEY:
    print("Set your APIKEY environment variable")
LEAGUE_LISTING = "http://api.steampowered.com/IDOTA2Match_570/GetLeagueListing/v0001?key=%s" % APIKEY


def apirequest(req_url):
    succeeded = False
    sleep_time = 1
    while not succeeded:
        try:
            time.sleep(sleep_time)
            print("Requesting: %s" % req_url)
            request = Request(req_url)
            request.add_header('User-Agent', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.87 Safari/537.36')
            response = urlopen(request)
            succeeded = True
        except Exception as e:
            sleep_time += 30  # incase script breaks dont want to spam
            print(e)
            print("Request failed. sleeping more")
            continue
    data = json.load(response)
    return data


def get_tournament_games(start_at=None):
    url = "http://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/v0001?key=%s&tournament_games_only=1"\
          % APIKEY
    if start_at:
        url += "&start_at_match_id=%d" % start_at
    return apirequest(url)


def add_new_replay(qmID, session):
    result = apirequest("https://api.opendota.com/api/replays?match_id=%d" % qmID)[0]
    replay_url = "http://replay{0}.valve.net/570/{1}_{2}.dem.bz2".format(
        result["cluster"], result["match_id"], result["replay_salt"]
    )
    session.execute("""INSERT into replayurls VALUES ('%d', '%s')""" % (qmID, replay_url))


def connect_postgres():
    connect_str = "dbname='replayurls'"
    # use our connection values to establish a connection
    conn = psycopg2.connect(connect_str)
    # create a psycopg2 cursor that can execute queries
    cursor = conn.cursor()
    return conn, cursor


def main():
    connection, session = connect_postgres()
    try:
        session.execute("""SELECT matchID from replayurls""")
    except:
        print("Select replayurls failed")
    existingIDs = [x[0] for x in session.fetchall()]
    start_at = None
    finished = False
    while not finished:
        games = get_tournament_games(start_at)
        queryMatchIDs = [m["match_id"] for m in games["result"]["matches"]]
        for i, qmID in enumerate(queryMatchIDs):
            if start_at and i == 0:
                continue  # due to start_at. not start next to
            if qmID not in existingIDs:
                add_new_replay(qmID, session)
                connection.commit()
            else:
                finished = True
        start_at = min(queryMatchIDs)
        print("New start: %d" % start_at)


if __name__ == "__main__":
    main()

