"use client"
import { useState, useEffect } from "react"
import axios from "axios";
import { log } from "console";


export default function Home() {
  const [todos, setTodos] = useState([])

  async function getTodos() {
    const resp = await axios.get("http://127.0.0.1:8000/api/todos")
    console.log(resp.data.todos);
    setTodos(resp.data.todos)
  }
  useEffect(() => {
    getTodos()
  }, [])

  return (
    <div className="">
      <div className="flex flex-col gap-4 items-center py-8">
        <div className="text-2xl">Todo List Rust</div>
        <div className="text-2xl">Todos</div>
        <div className="flex flex-col gap-6 bg-gray-200 p-4 rounded-md shadow-md">
          {todos && todos.map((todo, index) => {
            return (
              <div key={index} className="flex justify-between bg-blue-200 rounded-lg shadow-lg p-2">
                <div className="flex gap-2 items-center">
                  <input type="checkbox" readOnly />
                  <div className="text-xl">{todo.title}</div>
                </div>
                <div className="flex gap-4">
                  <button className="bg-green-500 p-2 hover:bg-green-600 text-lg rounded-lg text-white">Edit</button>
                  <button className="bg-red-500 p-2 hover:bg-red-600 text-lg rounded-lg text-white">Delete</button>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}
