"use client"
import React, { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion"
import { Textarea } from '@/components/ui/textarea'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Label } from '@/components/ui/label'
import { v4 } from "uuid"

function Todos() {
    const { toast } = useToast()
    const [todo, setTodo] = useState({
        id: "",
        title: "",
        desc: "",
        completed: false
    })
    const [todos, setTodos] = useState([])
    const [editMode, setEditMode] = useState(false)
    const [todoUpdate, setTodoUpdate] = useState({
        id: "",
        title: "",
        desc: "",
        completed: false
    })

    function addTodo() {
        if (todo.title != "") {
            setTodos(prevTodos => [...prevTodos, { ...todo, id: v4(), completed: false }])
            setTodo({ ...todo, title: "", desc: "" })
        }

    }

    function clearTodos() {
        setTodos([])
    }

    function editTodo(curTodo) {
        setEditMode(true)
        setTodoUpdate(curTodo)
    }

    function updateTodo() {
        setEditMode(false);
        setTodos((prevTodos) => {
            return prevTodos.map((t) => (t.id === todoUpdate.id ? todoUpdate : t));
        });
        setTodoUpdate({
            id: "",
            title: "",
            desc: "",
            completed: false,
        });
    }

    function deleteTodo(curTodo) {
        setTodos(prevTodos => {
            let curTodos = prevTodos.filter(t => t.id != curTodo.id)
            return curTodos
        })
    }

    return (
        <div className="flex flex-col gap-2 pt-4 items-center px-4">
            <div className='text-2xl'>Todo List shadcn</div>
            <Input
                value={todo.title}
                onChange={e => setTodo({ ...todo, title: e.target.value })}
                placeholder='Enter todo' />
            <Textarea
                value={todo.desc}
                onChange={e => setTodo({ ...todo, desc: e.target.value })}
                placeholder='Enter todo description' />
            <div className="flex gap-2">
                <Button
                    onClick={addTodo}
                    variant='default'>Add</Button>
                <Button
                    onClick={clearTodos}
                    variant='outline'>Clear</Button>
            </div>
            <div className="w-full">
                {todos &&
                    todos.map((curTodo, index) => {
                        return (
                            <Accordion type="single" collapsible className="">
                                <AccordionItem value="item-1">
                                    <div className="flex items-center justify-between">
                                        <AccordionTrigger>{curTodo.title}</AccordionTrigger>
                                        <div className="flex gap-2">
                                            <Dialog>
                                                <DialogTrigger asChild>
                                                    <Button onClick={() => editTodo(curTodo)} variant='green'>Edit</Button>
                                                </DialogTrigger>
                                                <DialogContent className="sm:max-w-[425px]">
                                                    <DialogHeader>
                                                        <DialogTitle>Edit Todo</DialogTitle>
                                                        <DialogDescription>
                                                            Make changes to your Todo. Click save when you're done.
                                                        </DialogDescription>
                                                    </DialogHeader>
                                                    <div className="grid gap-4 py-4">
                                                        <div className="grid grid-cols-4 items-center gap-4">
                                                            <Label htmlFor="name" className="text-right">
                                                                Title
                                                            </Label>
                                                            <Input id="title"
                                                                value={todoUpdate.title}
                                                                onChange={e => setTodoUpdate({ ...todoUpdate, title: e.target.value })}
                                                                className="col-span-3" />
                                                        </div>
                                                        <div className="grid grid-cols-4 items-center gap-4">
                                                            <Label htmlFor="username" className="text-right">
                                                                Description
                                                            </Label>
                                                            <Textarea id="description"
                                                                value={todoUpdate.desc}
                                                                onChange={e => setTodoUpdate({ ...todoUpdate, desc: e.target.value })}
                                                                className="col-span-3" />
                                                        </div>
                                                    </div>
                                                    <DialogFooter>
                                                        <Button type="submit" onClick={updateTodo}>Save changes</Button>
                                                    </DialogFooter>
                                                </DialogContent>
                                            </Dialog>
                                            <Button onClick={() => deleteTodo(curTodo)} variant='destructive'>Delete</Button>
                                        </div>
                                    </div>
                                    <AccordionContent>
                                        {curTodo.desc ? curTodo.desc : "No description."}
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        )
                    })}
            </div>
        </div >
    )
}

export default Todos