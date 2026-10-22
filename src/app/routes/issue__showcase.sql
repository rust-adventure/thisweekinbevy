SELECT
    issue.slug,
    issue.issue_date,
    issue.cloudinary_public_id,
    issue.display_name,
    issue.description,
    issue.youtube_id,
    showcases.showcases as "showcases: Json<Vec<ShowcaseData>>",
    crate_releases.crate_releases as "crate_releases: Json<Vec<SqlCrateRelease>>",
    devlogs.devlogs as "devlogs: Json<Vec<SqlDevlog>>",
    educationals.educationals as "educationals: Json<Vec<SqlEducational>>",
    new_github_issues as "new_github_issues: Json<Vec<SqlNewGhIssue>>",
    new_pull_requests as "new_pull_requests: Json<Vec<SqlNewPr>>",
    merged_pull_requests as "merged_pull_requests: Json<Vec<SqlMergedPullRequest>>"
FROM
    issue
    LEFT JOIN (
        SELECT
            issue.id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    showcase.title,
                    "url",
                    showcase.url,
                    "discord_url",
                    showcase.discord_url,
                    "description",
                    showcase.description,
                    "images",
                    images.data
                )
            ) as showcases
        FROM
            issue
            LEFT JOIN issue__showcase ON issue__showcase.issue_id = issue.id
            LEFT JOIN showcase on issue__showcase.showcase_id = showcase.id,
            LATERAL (
                SELECT
                    json_arrayagg(
                        json_object(
                            "cloudinary_public_id",
                            cloudinary_public_id,
                            "description",
                            description
                        )
                    ) as data
                FROM
                    showcase__image
                    LEFT JOIN image ON showcase__image.image_id = image.id
                WHERE
                    showcase.id = showcase_id
                GROUP BY
                    showcase.id
            ) as images
        WHERE
            issue_date = ?
        GROUP BY
            issue.id
    ) as showcases on showcases.id = issue.id
    LEFT JOIN (
        SELECT
            issue.id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    crate_release.title,
                    "url",
                    crate_release.url,
                    "discord_url",
                    crate_release.discord_url,
                    "description",
                    crate_release.description,
                    "images",
                    images.data
                )
            ) AS crate_releases
        FROM
            issue
            LEFT JOIN issue__crate_release ON issue__crate_release.issue_id = issue.id
            LEFT JOIN crate_release on issue__crate_release.crate_release_id = crate_release.id,
            LATERAL (
                SELECT
                    json_arrayagg(
                        json_object(
                            "cloudinary_public_id",
                            cloudinary_public_id,
                            "description",
                            description
                        )
                    ) as data
                FROM
                    crate_release__image
                    LEFT JOIN image ON crate_release__image.image_id = image.id
                WHERE
                    crate_release.id = crate_release_id
                GROUP BY
                    crate_release.id
            ) as images
        WHERE
            issue_date = ?
        GROUP BY
            issue.id
    ) as crate_releases ON crate_releases.id = issue.id
    LEFT JOIN (
        SELECT
            issue.id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    devlog.title,
                    "post_url",
                    devlog.post_url,
                    "video_url",
                    devlog.video_url,
                    "discord_url",
                    devlog.discord_url,
                    "description",
                    devlog.description,
                    "images",
                    images.data
                )
            ) AS devlogs
        FROM
            issue
            LEFT JOIN issue__devlog ON issue__devlog.issue_id = issue.id
            LEFT JOIN devlog on issue__devlog.devlog_id = devlog.id,
            LATERAL (
                SELECT
                    json_arrayagg(
                        json_object(
                            "cloudinary_public_id",
                            cloudinary_public_id,
                            "description",
                            description
                        )
                    ) as data
                FROM
                    devlog__image
                    LEFT JOIN image ON devlog__image.image_id = image.id
                WHERE
                    devlog.id = devlog_id
                GROUP BY
                    devlog.id
            ) as images
        WHERE
            issue_date = ?
        GROUP BY
            issue.id
    ) as devlogs ON devlogs.id = issue.id
    LEFT JOIN (
        SELECT
            issue.id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "title",
                    educational.title,
                    "post_url",
                    educational.post_url,
                    "video_url",
                    educational.video_url,
                    "discord_url",
                    educational.discord_url,
                    "description",
                    educational.description,
                    "images",
                    images.data
                )
            ) AS educationals
        FROM
            issue
            LEFT JOIN issue__educational ON issue__educational.issue_id = issue.id
            LEFT JOIN educational on issue__educational.educational_id = educational.id,
            LATERAL (
                SELECT
                    json_arrayagg(
                        json_object(
                            "cloudinary_public_id",
                            cloudinary_public_id,
                            "description",
                            description
                        )
                    ) as data
                FROM
                    educational__image
                    LEFT JOIN image ON educational__image.image_id = image.id
                WHERE
                    educational.id = educational_id
                GROUP BY
                    educational.id
            ) as images
        WHERE
            issue_date = ?
        GROUP BY
            issue.id
    ) as educationals ON educationals.id = issue.id,
    # git prs and issues
    LATERAL (
        SELECT
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
        WHERE
            issue.id = issue__new_github_issue.issue_id
        GROUP BY
            issue_id
    ) as new_github_issues,
    LATERAL (
        SELECT
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "github_id",
                    github_id,
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
        WHERE
            issue.id = issue__new_pull_request.issue_id
        GROUP BY
            issue_id
    ) as new_pull_requests,
    LATERAL (
        SELECT
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "github_id",
                    github_id,
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
        WHERE
            issue.id = issue__merged_pull_request.issue_id
        GROUP BY
            issue_id
    ) as merged_pull_requests
WHERE
    issue_date = ?;