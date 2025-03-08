import fetchAPI from "./functions/GetApi";
import { useEffect, useState } from "react";
import "./css/main.css";
import { motion, AnimatePresence } from "framer-motion";

function getCurrentLyrics(timestamp: number, lyrics: [string, number][]) {
  if (lyrics.length == 0) {
    return [];
  }
  for (let i = 0; i < lyrics.length; i++) {
    if (lyrics[i][1] > timestamp) {
      if (i > 0) {
        let length = lyrics[i][1] - lyrics[i - 1][1];
        let relative_progress = timestamp - lyrics[i - 1][1] + 500;

        let words = lyrics[i - 1][0].split(" ");

        let wordsTimeList = lyrics[i - 1][0].split(" ").map((word, index) => {
          let wordTime = (length / words.length) * index;
          return [word, wordTime];
        });

        return [wordsTimeList, relative_progress];
      }
    }
  }
  return [];
}

function App() {
  const [songData, setSongData] = useState({
    song: "",
    artist: "",
    album: "",
    cover: "",
    lyrics: [] as [string, number][],
    progress: 0,
  });

  useEffect(() => {
    if (songData.cover) {
      const favicon = document.getElementById("favicon") as HTMLLinkElement;
      if (favicon) {
        favicon.href = songData.cover;
      } else {
        const newFavicon = document.createElement("link");
        newFavicon.id = "favicon";
        newFavicon.rel = "icon";
        newFavicon.href = songData.cover;
        document.head.appendChild(newFavicon);
      }
    }
  }, [songData.cover]);

  useEffect(() => {
    const intervalId = setInterval(() => {
      (async () => {
        const song = await fetchAPI("current_song");

        if (song.song == "") {
          const url = await fetchAPI("");
          window.location.href = url.url;
        } else {
          setSongData((prevData) => ({
            song: song.song,
            artist: song.artist,
            album: song.album,
            cover: song.album_picture,
            progress: song.progress,
            lyrics: song.lyrics.length ? song.lyrics : prevData.lyrics,
          }));
        }
      })();
    }, 500);

    return () => clearInterval(intervalId);
  }, []);

  if (songData.song == "") {
    return <h1>Loading...</h1>;
  }

  const res = getCurrentLyrics(songData.progress, songData.lyrics);
  const current_lyrics: (string | number)[][] = Array.isArray(res[0])
    ? res[0]
    : [];
  const relative_progress = res[1];

  console.log(current_lyrics);

  return (
    <div className="App">
      <div className="left">
        <img
          className="cover"
          src={songData.cover}
          alt={`Album cover for ${songData.album}`}
        />
        <img
          className="cover"
          src={songData.cover}
          alt={`Album cover for ${songData.album}`}
        />
      </div>
      <div className="right lyrics">
        <AnimatePresence mode="popLayout">
          {current_lyrics.map((word, index) => (
            <motion.span
              key={`${word}-${index}`}
              initial={{ opacity: 0, filter: "blur(5px)" }}
              animate={{ opacity: 1, filter: "blur(0px)" }}
              exit={{ opacity: 0, y: -50, filter: "blur(5px)" }}
              transition={{ duration: 0.5 }}
              className={`current-lyric ${
                relative_progress > word[1] ? "active" : ""
              }`}
            >
              {word[0]}{" "}
            </motion.span>
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

export default App;
