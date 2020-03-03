(ns github-bot.github-api
  (:require [re-graph.core :as re-graph]
            [environ.core :refer [env]]))

(def token (env :github-oauth-token))  ;; OAuth token from `GITHUB_OAUTH_TOKEN' environment variable

(def api-url "https://api.github.com/graphql")

(def commit-query
  "query($owner: String!, $name: String!, $branch: String!) {
    repository(owner: $owner name: $name) {
      ref(qualifiedName: $branch) {
        target {
          ... on Commit {
            history(first: 5) {
              edges {
                node {
                  abbreviatedOid
                  messageHeadline
                  url
                  author {
                    user {
                      login
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }")

(re-graph/init {:http-url         api-url
                :http-parameters  {:with-credentials? false
                                   :oauth-token token}})
