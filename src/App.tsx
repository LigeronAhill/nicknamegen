import "./App.css";
import { CharacterForm } from "./components/form";

function App() {
  return (
    <main>
      <header>
        <h1 className="text-center text-3xl py-12 font-heading">
          Сгенерировать никнейм
        </h1>
      </header>
      <section className="flex flex-col items-center justify-center mx-auto font-sans">
        <CharacterForm />
      </section>
    </main>
  );
}

export default App;
