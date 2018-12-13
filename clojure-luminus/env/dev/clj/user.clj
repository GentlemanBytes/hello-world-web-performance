(ns user
  (:require [clojure-luminus.config :refer [env]]
            [clojure.spec.alpha :as s]
            [expound.alpha :as expound]
            [mount.core :as mount]
            [clojure-luminus.core :refer [start-app]]))

(alter-var-root #'s/*explain-out* (constantly expound/printer))

(defn start []
  (mount/start-without #'clojure-luminus.core/repl-server))

(defn stop []
  (mount/stop-except #'clojure-luminus.core/repl-server))

(defn restart []
  (stop)
  (start))


