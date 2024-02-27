SELECT
    slug,
    issue_date,
    cloudinary_public_id,
    display_name,
    description,
    youtube_id,
    showcases as "showcases: Json<Vec<ShowcaseData2>>",
    new_github_issues as "new_github_issues: Json<Vec<SqlNewGhIssue>>",
    new_pull_requests as "new_pull_requests: Json<Vec<SqlNewPr>>",
    merged_pull_requests as "merged_pull_requests: Json<Vec<SqlMergedPullRequest>>"
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
    LEFT JOIN (
        SELECT
            issue_id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    title,
                    "url",
                    url,
                    "gh_created_at",
                    gh_created_at,
                    "author",
                    author,
                    "author_url",
                    author_url
                )
            ) AS new_github_issues
        FROM
            issue__new_github_issue
            LEFT JOIN new_github_issue ON new_github_issue.id = issue__new_github_issue.github_issue_id
        GROUP BY
            issue_id
    ) AS ngis ON ngis.issue_id = issue.id
    LEFT JOIN (
        SELECT
            issue_id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    title,
                    "url",
                    url,
                    "gh_created_at",
                    gh_created_at,
                    "author",
                    author,
                    "author_url",
                    author_url
                )
            ) AS new_pull_requests
        FROM
            issue__new_pull_request
            LEFT JOIN new_pull_request ON new_pull_request.id = issue__new_pull_request.pull_request_id
        GROUP BY
            issue_id
    ) AS new_prs ON new_prs.issue_id = issue.id
    LEFT JOIN (
        SELECT
            issue_id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    title,
                    "url",
                    url,
                    "merged_at_date",
                    merged_at_date,
                    "author",
                    author,
                    "author_url",
                    author_url
                )
            ) AS merged_pull_requests
        FROM
            issue__merged_pull_request
            LEFT JOIN merged_pull_request ON merged_pull_request.id = issue__merged_pull_request.merged_pull_request_id
        GROUP BY
            issue_id
    ) AS merged_prs ON merged_prs.issue_id = issue.id
WHERE
    issue_date = ?;