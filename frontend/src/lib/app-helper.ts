 
import serverConfig from  '@/config/server-config.json' 

const NODE_ENV = process.env.NODE_ENV ? process.env.NODE_ENV : 'development'


export function getEnvironmentName(): string {
  const envName = NODE_ENV ? NODE_ENV : 'development'

  return envName
}


export function getBackendServerUrl(){


  const ENV_MODE = getEnvironmentName()

  //@ts-ignore
  const localServerConfig:any = serverConfig[ENV_MODE]

  const backendServerUrl = localServerConfig.backendServerUrl

  return backendServerUrl

}

export function getRpcServerUrl(){

  const ENV_MODE = getEnvironmentName()

  //@ts-ignore
  const localServerConfig:any = serverConfig[ENV_MODE]

  const rpcServerUrl = localServerConfig.rpcServerUrl

  return rpcServerUrl



}