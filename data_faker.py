import psycopg2
from faker import Faker
from itertools import cycle
import time

NUM_AUTHORS = 300
NUM_POSTS = 30000
TIMEOUT = .5 # minutes

fake = Faker()


start_time = time.time()
wait_time = 1
while True:
    if time.time() - start_time > TIMEOUT * 60:
        exit("timed out waiting for connection")
    try:
        conn = psycopg2.connect("postgres://rust:password@localhost:5432/rustdb")
        break
    except psycopg2.OperationalError:
        print(f"could not connect to db, sleeping for {wait_time} seconds")
        wait_time = wait_time * 2
        time.sleep(wait_time)



cur = conn.cursor()
cur.execute("DELETE FROM author")
cur.execute("ALTER SEQUENCE author_authorid_seq RESTART WITH 1")
cur.execute("DELETE FROM post")
cur.execute("ALTER SEQUENCE post_postid_seq RESTART WITH 1")
cur.execute("DELETE FROM author_post")
cur.execute("ALTER SEQUENCE author_post_id_seq RESTART WITH 1")

authors = []
for i in range(NUM_AUTHORS):
    authors.append((
        fake.user_name(),
        fake.first_name(),
        fake.last_name(),
        fake.email()
    ))
print("Inserting authors")
cur.executemany(
    "INSERT INTO author (username, firstname, lastname, email) VALUES (%s, %s, %s, %s)",
    authors
)

cur.execute("SELECT authorid FROM author")
authorids = [j for i in cur.fetchall() for j in i]

posts = []
for i in range(NUM_POSTS):
    posts.append((
        fake.sentence(),
        fake.paragraphs(),
        fake.date_time_this_year(),
        fake.date_time_this_year()
    ))
print("Inserting posts")
cur.executemany(
    "INSERT INTO post (title, body, created, updated) VALUES (%s, %s, %s, %s)",
    posts
)
cur.execute("SELECT postid FROM post")
postids = [j for i in cur.fetchall() for j in i]

author_post = list(zip(cycle(authorids), postids))

print("Inserting authors and posts")
cur.executemany(
    "INSERT INTO author_post (authorid, postid) VALUES (%s, %s)",
    author_post
)

conn.commit()