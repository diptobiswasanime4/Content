import { Button } from '@/components/ui/button'
import React from 'react'

function Hello() {
    return (
        <div className="h-full">
            <div className="flex flex-col gap-2 justify-center items-center">
                <div>Hello</div>
                <Button variant="default">Btn</Button>
            </div>
        </div>
    )
}

export default Hello