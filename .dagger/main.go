// A generated module for TestWorkflows functions
//
// This module has been generated via dagger init and serves as a reference to
// basic module structure as you get started with Dagger.
//
// Two functions have been pre-created. You can modify, delete, or add to them,
// as needed. They demonstrate usage of arguments and return types using simple
// echo and grep commands. The functions can be called from the dagger CLI or
// from one of the SDKs.
//
// The first line in this comment block is a short description line and the
// rest is a long description with more detail on the module's purpose or usage,
// if appropriate. All modules should have a short description.

package main

import (
	"context"
	"dagger/test-workflows/internal/dagger"
)

type TestWorkflows struct{}

// Starts a vault server and puts a secret in it
// docker run --rm --cap-add=IPC_LOCK -p 8200:8200 -e 'VAULT_DEV_ROOT_TOKEN_ID=myroot' -e 'VAULT_ADDR=http://127.0.0.1:8200' --name=dev-vault hashicorp/vault
func (m *TestWorkflows) PrepareVault(ctx context.Context) (*dagger.Service, error) {

	rootToken := dag.SetSecret("rootToken", "justademo")

	vaultContainer := dag.Container().
		From("hashicorp/vault:1.19")

	svc := vaultContainer.
		WithSecretVariable("VAULT_DEV_ROOT_TOKEN_ID", rootToken).
		WithEnvVariable("VAULT_ADDR", "http://0.0.0.0:8200"). // for the cli
		WithExposedPort(8200).
		AsService()

	prepped := vaultContainer.
		WithServiceBinding("vault-dev", svc).
		WithEnvVariable("VAULT_ADDR", "http://vault-dev:8200").
		WithSecretVariable("VAULT_TOKEN", rootToken).
		WithExec([]string{"vault", "kv", "put", "-mount=secret", "foo", "bar=baz"})
	println(prepped.Stdout(ctx))

	// svc.Up() // ?

	return svc, nil
}


// Deploy shows an example of reading a secret directly from vault in the
// code
func (m *TestWorkflows) Deploy(ctx context.Context, deployKey *dagger.Secret) (*dagger.Container, error) {

	// Suppose we need to deploy, and we need a secret to account our

	// This does not work for some reason... I guess because we're in a module?
	// secret := dag.Secret("vault://secret.foo.bar")
	secret := deployKey

	ctr := dag.Container().
		From("alpine:latest").
		WithSecretVariable("DEPLOYMENT_KEY", secret).
		WithExec([]string{"sh", "-c", "echo $DEPLOYMENT_KEY"})
	return ctr, nil
}
