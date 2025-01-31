import type { ApiOptions } from "@polkadot/api/types";
import type { RegistryTypes } from "@polkadot/types/types";
import { TypeRegistry } from "@polkadot/types";

import { typeDefinitions } from '@polkadot/types';
import { types } from './lib/types';

export const phalaRegistryTypes = { ...types, ...typeDefinitions } as unknown as RegistryTypes

export const phalaTypes = new TypeRegistry()

phalaTypes.register(phalaRegistryTypes)


export function options(options: ApiOptions = {}): ApiOptions {
  return {
    ...options,
    types: {
      ...phalaRegistryTypes,
      ...options.types || {},
    } as unknown as ApiOptions['types'],
  }
}

