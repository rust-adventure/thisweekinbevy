SELECT
    slug,
    issue_date,
    cloudinary_public_id,
    display_name,
    description,
    youtube_id,
    showcases as "showcases: Json<Vec<ShowcaseData2>>"
FROM
    issue
    LEFT JOIN (
        SELECT
            issue_id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    scase.title,
                    "url",
                    scase.url,
                    "discord_url",
                    scase.discord_url,
                    "description",
                    scase.description,
                    "images",
                    scase.images
                )
            ) AS showcases
        FROM
            issue__showcase
            LEFT JOIN (
                SELECT
                    id,
                    title,
                    url,
                    description,
                    discord_url,
                    images
                from
                    showcase
                    LEFT JOIN (
                        SELECT
                            showcase_id,
                            JSON_ARRAYAGG(
                                JSON_OBJECT(
                                    "id",
                                    TO_BASE64(image.id),
                                    "description",
                                    description,
                                    "cloudinary_public_id",
                                    cloudinary_public_id
                                )
                            ) AS images
                        FROM
                            showcase__image
                            LEFT JOIN image ON showcase__image.image_id = image.id
                        GROUP BY
                            showcase_id
                    ) as i on i.showcase_id = showcase.id
            ) as scase ON scase.id = issue__showcase.showcase_id
        GROUP BY
            issue_id
    ) AS s ON s.issue_id = issue.id
WHERE
    issue_date = ?;