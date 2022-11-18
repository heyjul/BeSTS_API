-- Add migration script here
CREATE VIEW score (
    user_id,
    room_id, 
    score
)
AS
SELECT
    user_id,
    room_id,
    SUM(
        CASE
            WHEN bet.team_one_score = result.team_one_score 
                    AND bet.team_two_score = result.team_two_score
                THEN match.guess_points
            WHEN (bet.team_one_score > bet.team_two_score
                    AND result.team_one_score > result.team_two_score)
                OR
                (bet.team_one_score < bet.team_two_score
                    AND result.team_one_score < result.team_two_score)
                THEN match.winner_points
            ELSE 0
        END
    )
FROM
    bet
    JOIN match ON bet.match_id = match.id
    JOIN result ON match.id = result.match_id
    JOIN room ON match.room_id = room.id
GROUP BY 
    user_id,
    room_id;
