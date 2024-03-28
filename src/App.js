import { appDataDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import React, { useEffect, useState } from "react";
import "./App.css"; // Import the CSS file

const NotesApp = () => {
  const [notes, setNotes] = useState([]);
  const [newNote, setNewNote] = useState("");

  useEffect(() => {
    loadNotes();
  }, []);

  const loadNotes = async () => {
    const appDataPath = await appDataDir();
    const notesData = await invoke("load_notes", { appDataPath });
    setNotes(notesData);
  };

  const addNote = async () => {
    if (newNote.trim() !== "") {
      const appDataPath = await appDataDir();
      await invoke("add_note", { appDataPath, note: newNote });
      setNewNote("");
      loadNotes();
    }
  };

  const deleteNote = async (index) => {
    const appDataPath = await appDataDir();
    await invoke("delete_note", { appDataPath, index });
    loadNotes();
  };

  return (
    <div className="application-container">
      <div className="app-container">
        <h1 className="heading">Notes App</h1>
        <div
          style={{
            marginBottom: "20px",
            display: "flex",
            alignItems: "center",
          }}
        >
          <input
            type="text"
            value={newNote}
            onChange={(e) => setNewNote(e.target.value)}
            placeholder="Add a new note..."
            className="input-field"
          />
          <button onClick={addNote} className="add-note-button">
            Add Note
          </button>
        </div>
        <ul className="notes-list">
          {notes.map((note, index) => (
            <li key={index} className="note-item">
              <span className="note-text">{note}</span>
              <button
                onClick={() => deleteNote(index)}
                className="delete-button"
              >
                Delete
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default NotesApp;
