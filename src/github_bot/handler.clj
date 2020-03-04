(ns github-bot.handler
  (:require [clojure.string :as str]
            [compojure.core :refer :all]
            [compojure.route :as route]
            [ring.middleware.json :refer [wrap-json-body]]
            [re-graph.core :as re-graph]
            [environ.core :refer [env]]
            [morse.handlers :refer :all]
            [morse.api :as tg]
            [github-bot.github-api :as gh])
  (:gen-class))

(def token (env :telegram-token))  ;; telegram token from `TELEGRAM_TOKEN' environment variable
(def chat-id (env :chat-id))       ;; telegram chat id from `CHAT_ID' environment variable

(tg/set-webhook token (env :webhook-url))

(defn format-gh-push-message [data]
  (let [{:keys [ref repository commits]} data]
    (str "🔨 " (count commits) (if (= (count commits) 1) " uusi commit" " uutta committia") " - " (repository :full_name) ":" (str/replace ref #"refs/heads/" "")
         (apply str (mapcat (fn [{:keys [id author message url]}]
                              (->>
                               (into [(str "<a href='" url "'>" (subs id 0 7) "</a> (" (author :username) "): " message)])
                               (into ["\n\n"]))) commits)))))

(defn format-gh-commits-message [repo data]
  (let [{{{{{commits :edges} :history} :target} :ref} :repository} data]
    (str "🔨 Viimeisimmät commitit - " repo ":"
         (apply str (mapcat (fn [{:keys [node]}]
                              (->>
                               (into [(str "<a href='" (node :url) "'>" (node :abbreviatedOid) "</a> (" (get-in node [:author :user :login]) "): " (node :messageHeadline))])
                               (into ["\n\n"]))) commits)))))

(def start-reply "Botti käynnistetty.")
(def help-reply "Komennot: 

/start - käynnistä botti
/help - näytä tämä viesti
/commits [repo] (esim. `/commits prodeko-org-djangocms)`")

(defn send-tg [message]
  (tg/send-text token chat-id {:parse_mode "HTML" :disable_web_page_preview true} message))

(defhandler telegram-handler
  (command "start" {}
           (send-tg start-reply))

  (command "help" {}
           (send-tg help-reply))

  (command "commits" {text :text}
           (let [repo (second (str/split text #" "))]
             (re-graph/query gh/commit-query
                             {:owner "Prodeko" :name repo :branch "master"}
                             (fn [{:keys [data errors]}]
                               (if errors
                                 (send-tg (str "Repoa <b>" repo "</b> ei löytynyt."))
                                 (send-tg (format-gh-commits-message repo data)))))
             "OK"))
  (message message "OK"))

(defn github-handler [data]
  (let [message (format-gh-push-message data)]
    (send-tg message)))

(defroutes app-routes
  (POST "/telegram" {body :body} (telegram-handler body))
  (POST "/github" {body :body} (github-handler body))
  (route/not-found "Not Found"))

(def app
  (wrap-json-body app-routes {:keywords? true}))