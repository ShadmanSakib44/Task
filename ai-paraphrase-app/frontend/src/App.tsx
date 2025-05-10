import { useState } from 'react'
import { useMediaQuery } from 'react-responsive'
import './App.css'

function App() {
  const [text, setText] = useState('')
  const [selection, setSelection] = useState<[number, number] | null>(null)

  const isMobile = useMediaQuery({ maxWidth: 767 })

  const handleTextSelect = () => {
    const textarea = document.getElementById('text-editor') as HTMLTextAreaElement
    const start = textarea.selectionStart
    const end = textarea.selectionEnd
    setSelection(start !== end ? [start, end] : null)
  }

  const handleParaphrase = async () => {
    if (!selection) {
      alert('Please select some text to paraphrase.')
      return
    }

    const [start, end] = selection
    const selectedText = text.slice(start, end)

    try {
      const res = await fetch(`${import.meta.env.VITE_API_URL}/paraphrase`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: selectedText }),
      })

      const data = await res.json()
      const paraphrased = data.result || 'Could not paraphrase.'

      const newText = text.slice(0, start) + paraphrased + text.slice(end)
      setText(newText)
      setSelection(null)
    } catch (err) {
      console.error(err)
      alert('Something went wrong with the paraphrasing API.')
    }
  }

  return (
    <div className="container">
      <h1>📝 AI Text Paraphraser</h1>
      <p className="subheading">Select any part of your text and click "Paraphrase"</p>

      <textarea
        id="text-editor"
        className="editor"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onMouseUp={handleTextSelect}
        onKeyUp={handleTextSelect}
        placeholder="Start typing or paste your text here..."
      />

      <button
        onClick={handleParaphrase}
        disabled={!selection}
        className="paraphrase-btn"
      >
        {isMobile ? 'Paraphrase' : 'Paraphrase Selected Text'}
      </button>

      {/* Footer */}
      <footer className="footer">
        Built by <a href="https://sakibshadman.com" target="_blank" rel="noopener noreferrer">sakibshadman.com</a>
      </footer>
    </div>
  )
}

export default App
