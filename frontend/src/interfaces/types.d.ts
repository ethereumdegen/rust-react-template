
export type AssertionResult<T>  =  AssertionSuccess<T> | AssertionFailure
export interface AssertionSuccess<T> {
    success: boolean
    data: T
  }
  
  export interface AssertionFailure {
    success: boolean
    error: string
  }
  
 
  
  export interface DomainData {
    name: string
    version: string
    chainId: number
    verifyingContract: string
  }
   