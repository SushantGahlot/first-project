                SELECT
                    post.postid,
                    post.title,
                    array_agg(author.authorid) as authorids,
                    array_agg(author.firstname) as firstnames,
                    array_agg(author.username) as usernames,
                    arrar_agg(author.lastname) as lastnames,
                    array_agg(author.email) as emails,
                FROM
                    author_post
                    INNER JOIN post ON author_post.postid = post.postid
                    INNER JOIN author ON author_post.authorid = author.authorid
                WHERE
                    author.authorid = ANY(ARRAY[1,2,3,4,5,6,7,8,9,10])
                GROUP BY
                    post.postid,
                    post.title,
                    author.authorid,
                    author.firstname,
                    author.username,
                    author.lastname,
                    author.email