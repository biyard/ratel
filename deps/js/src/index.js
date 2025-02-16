import { initializeApp } from "firebase/app";
import {
  GoogleAuthProvider,
  getAuth,
  signInWithPopup,
  signOut,
} from "firebase/auth";

const firebase = {
  initializeApp: (conf) => {
    console.log(conf);
    initializeApp(conf);
  },
  getAuth,
  signInWithPopup,
  signOut,
  GoogleAuthProvider,
};

window.firebase = firebase;
