(defproject github-bot "0.1.1-prodeko-github-bot"
  :description "Post Github commits to Prodeko Webteam chat"
  :url "http://github.com/Prodeko/github-telegram-bot"

  :license {:name "Eclipse Public License"
            :url "http://www.eclipse.org/legal/epl-v10.html"}

  :dependencies [[org.clojure/clojure "1.10.3"]
                 [ring/ring-json "0.5.1"]
                 [ring/ring-core "1.9.4"]
                 [compojure "1.6.2"]
                 [environ "1.2.0"]
                 [morse "0.4.3"]
                 [re-graph "0.1.16"]
                 [org.eclipse.jetty/jetty-server "9.4.31.v20200723"]]

  :plugins [[lein-environ "1.2.0"]
            [lein-ring "0.12.6"]]

  :ring {:handler github-bot.handler/app
         :port 3000
         :nrepl {:start? true
                 :port 55444}}

  :target-path "target/%s"

  :profiles {:dev     {:dependencies [[javax.servlet/servlet-api "2.5"]
                                      [ring/ring-mock "0.4.0"]]}})
