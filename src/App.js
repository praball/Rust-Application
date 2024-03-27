import { appDataDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';

const NotesApp = () => {
  const [notes, setNotes] = useState([]);
  const [newNote, setNewNote] = useState('');

  useEffect(() => {
    loadNotes();
  }, []);

  const loadNotes = async () => {
    const appDataPath = await appDataDir();
    const notesData = await invoke('load_notes', { appDataPath });
    setNotes(notesData);
  };

  const addNote = async () => {
    if (newNote.trim() !== '') {
      const appDataPath = await appDataDir();
      await invoke('add_note', { appDataPath, note: newNote });
      setNewNote('');
      loadNotes();
    }
  };

  const deleteNote = async (index) => {
    const appDataPath = await appDataDir();
    await invoke('delete_note', { appDataPath, index });
    loadNotes();
  };

  return (
    <div>
      <h1>Notes App</h1>
      <div>
        <input
          type="text"
          value={newNote}
          onChange={(e) => setNewNote(e.target.value)}
          placeholder="Add a new note..."
        />
        <button onClick={addNote}>Add Note</button>
      </div>
      <ul>
        {notes.map((note, index) => (
          <li key={index}>
            {note}
            <button onClick={() => deleteNote(index)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default NotesApp;
