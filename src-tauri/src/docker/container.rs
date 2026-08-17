use crate::docker::image::BcImage;
use bollard::config::ContainerCreateBody;
use bollard::plugin::ContainerCreateResponse;
use bollard::query_parameters::{CreateContainerOptions, ListImagesOptionsBuilder};

pub struct BcContainer {
    id: String,
    name: String,
}

impl BcContainer {
    pub async fn start(&self) {}
    pub async fn stop(&self) {}
}

pub struct ContainerBuilder {
    docker: bollard::Docker,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        ContainerBuilder {
            docker: bollard::Docker::connect_with_defaults().unwrap(),
        }
    }

    pub async fn build(
        &self,
        image: &BcImage,
        container_name: &str,
    ) -> Result<BcContainer, ContainerError> {
        let options = ListImagesOptionsBuilder::default().all(true).build();
        let images = self.docker.list_images(Some(options)).await?;
        let image_ids: Vec<String> = images.iter().map(|i| i.id.clone()).collect(); // TODO redo
        if !image_ids.contains(&image.id().to_string()) {
            return Err(ContainerError::ImageNotFound(image.id().to_string()));
        }
        let create_response = self.create_container(image, container_name).await?;

        Ok(BcContainer {
            id: create_response.id,
            name: String::from("LOL"),
        })
    }

    pub async fn create_container(
        &self,
        image: &BcImage,
        container_name: &str,
    ) -> Result<ContainerCreateResponse, ContainerError> {
        let mut options = CreateContainerOptions::default();
        options.name = Some(container_name.to_string());
        let mut config = ContainerCreateBody::default();
        config.image = Some(image.id().to_string());

        let create_response = self.docker.create_container(Some(options), config).await?;

        Ok(create_response)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("docker causes an error: {0}")]
    BollardOperation(#[from] bollard::errors::Error),

    #[error("could not find image {0} in docker")]
    ImageNotFound(String),
}
