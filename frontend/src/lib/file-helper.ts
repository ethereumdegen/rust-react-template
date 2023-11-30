import fs from 'fs'
import path from 'path' 
 
export function readJSONFile(uri: string): any {
  const input = fs.readFileSync(path.resolve(uri), {
    encoding: 'utf8',
    flag: 'r',
  })

  return JSON.parse(input)
}

